use openssl::pkey::{PKey,Private};
use crate::client::email::Email;
// use tokio_native_tls::TlsStream;
// use tokio::net::TcpStream;
use crate::client::{io,ClientEmailError};

mod connect;

pub use connect::Connected;

#[derive(Default,Debug)]
pub struct Features{
    pub result:bool,
    pub start_tls:bool,
    pub limit_size:bool,
    pub size:u64,
    pub pipeline:bool,
    pub chunking:bool,
    pub help:bool
}

pub struct Connection{
    pub domain:String,
    pub server_name:String,
    pub private_key:PKey<Private>,
    pub dkim_selector:String,
    pub emails:Vec<Email>,
    // pub verify:Vec<String>,
    pub sender_domain:String,
    pub enable_danger_accept:bool
}

#[allow(dead_code)]
impl Connection{
    pub fn new(
        domain:String,
        server_name:String,
        key:String,
        dkim_selector:String,
        sender_domain:String
    )->Result<Connection,&'static str>{
        let private_key:PKey<Private>;
        match PKey::private_key_from_pem(&key.into_bytes()){
            Ok(k)=>{
                private_key = k;
            },
            Err(_e)=>{
                return Err("failed-invalid_key");
            }
        }
        return Ok(Connection{
            domain:domain,
            server_name:server_name,
            private_key:private_key,
            dkim_selector:dkim_selector,
            emails:vec![],
            // verify:vec![],
            sender_domain:sender_domain,
            enable_danger_accept:false
        });
    }
    pub fn add(&mut self,email:Email){self.emails.push(email);}
    // #[allow(dead_code)]
    // pub fn verify(&mut self,email:String){self.verify.push(email);}
    // #[allow(dead_code)]
    // pub async fn verify_all(&mut self)->Result<(),&'static str>{
    //     match verify_all(self).await{
    //         Ok(_v)=>{return Ok(());},
    //         Err(e)=>{return Err(e);}
    //     }
    // }
    pub fn enable_danger_accept(&mut self,v:bool){if v{self.enable_danger_accept = true;}}
    pub async fn send(&mut self)->Result<(Vec<String>,Vec<(String,ClientEmailError)>),&'static str>{
        match send_emails(self).await{
            Ok(_v)=>{return Ok(_v);},
            Err(e)=>{return Err(e);}
        }
    }
}

async fn send_emails(config:&mut Connection)->Result<(Vec<String>,Vec<(String,ClientEmailError)>),&'static str>{

    // println!("### called-send_emails");

    let mut parsed_mails:Vec<(Vec<String>,String,u64)> = Vec::new();
    loop{
        if config.emails.len() == 0{
            break;
        }
        let email = config.emails.remove(0);
        match email.parse(&config).await{
            Ok(v)=>{
                parsed_mails.push(v);
            },
            Err(_)=>{
                println!("failed-parse_email");
                return Err("failed-parse_email");
            }
        }
    }

    // println!("### parsed-send_emails");

    let mut connection:Connected;
    let features:Features;
    match build_smtp_connection(config).await{
        Ok((conn,f))=>{
            connection = conn;
            features = f;
        },
        Err(_e)=>{
            return Err(_e);
        }
    }

    // features.pipeline = false;

    // println!("### connected-send_emails");

    if features.limit_size{
        for (_,_,size) in parsed_mails.iter(){
            if size > &features.size{
                return Err("max-size_limit-reached");
            }
        }
    }

    // println!("### checked-send_emails");

    // println!("\n\n### features : {:?}\n\n",features);

    //parse email
    let mut failed = Vec::new();
    let mut successfull = Vec::new();
    loop{
        if parsed_mails.len() == 0{
            break;
        }
        let (mut commands,tracking_id,_) = parsed_mails.remove(0);
        match process_mail(&mut connection,&mut commands,&features).await{
            Ok(_)=>{
                successfull.push(tracking_id);
            },
            Err(_e)=>{
                // println!("### send_mail_error : {:?}",_e);
                failed.push((tracking_id,_e));
                match io::secure_send_with_response(&mut connection,"RSET\r\n".to_string()).await{
                    Ok(response)=>{
                        if !response.result{
                            return Err("reset_failed");
                        }
                    },
                    Err(_)=>{
                        return Err("reset-request-failed");
                    }
                }
            }
        }
    }

    match io::secure_send(&mut connection,"QUIT\r\n".to_string()).await{
        Ok(_)=>{},
        Err(_)=>{}
    }

    return Ok((successfull,failed));

}

pub async fn process_mail(connection:&mut Connected,commands:&mut Vec<String>,features:&Features)->Result<(),ClientEmailError>{

    // println!("### process_mail");

    let body:String;
    match commands.pop(){
        Some(v)=>{body = v;},
        None=>{return Err(ClientEmailError::InvalidBody);}
    }
    let batch_commands_len = commands.len();

    let mut to_allowed = 0;
    let mut index = 0;
    loop{

        if commands.len() == 0{
            break;
        }

        if commands.len() == 1 && !features.pipeline{
            if to_allowed == 0{
                return Err(ClientEmailError::To);
            }
        }

        let command = commands.remove(0);

        // println!("\n\n command : {:?}\n\n",command);

        match io::secure_send(connection,command).await{
            Ok(_)=>{},
            Err(_)=>{
                // println!("### failed-email_command-send");
                return Err(ClientEmailError::ConnectionError);
            }
        }

        // println!("### command_sent");
        
        if !features.pipeline{
            match io::secure_read(connection).await{
                Ok(response)=>{
                    // println!("response : {:?} {:?}\n\n",response,commands.len());
                    if !response.result{
                        //from command
                        if index == 0{
                            return Err(ClientEmailError::From);
                        }
                        if commands.len() == 0{
                            return Err(ClientEmailError::Data);
                        }
                        // println!("### non-pipeline response : {:?}",response);
                        // return Err(ClientEmailError::ConnectionError);
                    } else {
                        if index > 0 && commands.len() > 0{
                            to_allowed += 1;
                        }
                    }
                },
                Err(_e)=>{
                    println!("### failed-non_pipeline-read : {:?}",_e);
                    return Err(ClientEmailError::ConnectionError);
                }
            }
        }

        index += 1;

    }

    if to_allowed == 0 && !features.pipeline{
        return Err(ClientEmailError::To);
    }

    if features.pipeline{
        index = 0;
        let mut to_command = 0;
        loop{
            if index == batch_commands_len{
                // println!("responses finished");
                break;
            }
            match io::secure_read_qued(connection).await{
                Ok(responses)=>{
                    for response in responses{
                        // println!("### response : {:?} {:?}",index,response.result);
                        if index == 0{
                            if !response.result{
                                return Err(ClientEmailError::From);
                            }
                        }
                        if index > 0 && index < batch_commands_len-1{
                            // println!("to command");
                            if response.result{
                                to_command += 1;
                            }
                        }
                        if index == batch_commands_len-1{
                            if !response.result{
                                return Err(ClientEmailError::Data);
                            }
                        }
                        index += 1;
                    }
                },
                Err(_)=>{
                    return Err(ClientEmailError::ConnectionError);
                }
            }//process responses   
        }//loop all tyhe repsonses
        if to_command == 0{
            println!("to to to to to");
            return Err(ClientEmailError::To);
        }
    }

    // println!("body.contains : {:?}",body.contains("\r\n.\r\n"));

    // let k = body.clone();
    // loop{
    //     if k.contains("\r\n.\r\n"){
    //         k = k.replace("\r\n.\r\n","");
    //         println!("replaced");
    //     } else {
    //         break;
    //     }
    // }

    // println!("\n--------\\n{}\n--------\n",k);

    //send body
    match io::secure_send(connection,body).await{
        Ok(_)=>{},
        Err(_)=>{
            // println!("### failed-email_command-send");
            return Err(ClientEmailError::ConnectionError);
        }
    }

    //read body response
    match io::secure_read_qued(connection).await{
        Ok(responses)=>{
            for response in responses{
                // println!("### response : {:?}",response);
                if !response.result{
                    return Err(ClientEmailError::Body);
                }
            }
        },
        Err(_)=>{
            return Err(ClientEmailError::ConnectionError);
        }
    }

    return Ok(());

}

pub async fn build_smtp_connection(config:&mut Connection)->Result<(Connected,Features),&'static str>{

    // println!("### called-build_smtp_connection");

    let mut connection:Connected;
    let port:u32;
    match connect::init(config).await{
        Ok((c,p))=>{connection = c;port = p;},
        Err(_e)=>{
            return Err("failed-connect");
        }
    }

    // println!("### connection_init-build_smtp_connection");

    //wait for helo
    match io::secure_read(&mut connection).await{
        Ok(read)=>{
            if !read.result{
                return Err("denied-wait_for_hello");
            }
        },
        Err(_e)=>{
            println!("### failed-wait_for_hello : {:?}",_e);
            return Err("failed-wait_for_hello");
        }
    }

    // println!("### hello_received-build_smtp_connection");

    let features:Features;
    match io::secure_send_with_features(&mut connection,format!("EHLO {}\r\n",config.server_name)).await{
        Ok(response)=>{
            if !response.result{
                match io::secure_send_with_features(&mut connection,format!("HELO {}\r\n",config.server_name)).await{
                    Ok(response)=>{
                        if !response.result{
                            return Err("failed-HELO&EHLO");
                        } else {
                            features = response
                        }
                    },
                    Err(_e)=>{
                        // println!("### failed-reply_hello : {:?}",_e);
                        return Err("failed-send-HELO");
                    }
                }
            } else {
                features = response;
            }
        },
        Err(_e)=>{
            // println!("### failed-send-EHLO : {:?}",_e);
            return Err("failed-send-EHLO");
        }
    }

    // println!("### features_parsed-build_smtp_connection");

    if features.start_tls{
        match connection{
            Connected::InSecure(_)=>{
                match io::secure_send_with_response(&mut connection,format!("STARTTLS\r\n")).await{
                    Ok(response)=>{
                        if response.result{
                            match connect::start_tls(connection, config.domain.clone(), &port).await{
                                Ok(v)=>{
                                    // println!("###### start_tls complete");
                                    connection = v;
                                },
                                Err(_)=>{
                                    return Err("failed-start-tls");
                                }
                            }
                        }
                    },
                    Err(_e)=>{
                        println!("### failed-reply_hello : {:?}",_e);
                        return Err("failed-wait_for_hello");
                    }
                }
            },
            _=>{}
        }
    }

    // println!("### connection_built-build_smtp_connection");

    return Ok((connection,features));

}