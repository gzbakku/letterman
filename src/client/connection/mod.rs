use openssl::pkey::{PKey,Private};
use crate::client::email::Email;
// use tokio_native_tls::TlsStream;
// use tokio::net::TcpStream;
use crate::client::io;

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
    pub sender_domain:String,
}

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
            sender_domain:sender_domain
        });
    }
    pub fn add(&mut self,email:Email){
        self.emails.push(email);
    }
    pub async fn send(&mut self)->Result<(Vec<String>,Vec<String>),&'static str>{
        match build(self).await{
            Ok(_v)=>{return Ok(_v);},
            Err(e)=>{return Err(e);}
        }
    }
}

async fn build(config:&mut Connection)->Result<(Vec<String>,Vec<String>),&'static str>{

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

    let mut connection:Connected;
    let port:u32;
    match connect::init(config).await{
        Ok((c,p))=>{connection = c;port = p;},
        Err(_e)=>{
            return Err("failed-connect");
        }
    }

    //wait for helo
    match io::secure_read(&mut connection).await{
        Ok(read)=>{
            if !read.result{
                return Err("denied-wait_for_hello");
            }
        },
        Err(_e)=>{
            println!("!!! failed-wait_for_hello : {:?}",_e);
            return Err("failed-wait_for_hello");
        }
    }

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
                        // println!("!!! failed-reply_hello : {:?}",_e);
                        return Err("failed-send-HELO");
                    }
                }
            } else {
                features = response;
            }
        },
        Err(_e)=>{
            println!("!!! failed-send-EHLO : {:?}",_e);
            return Err("failed-send-EHLO");
        }
    }

    if features.limit_size{
        for (_,_,size) in parsed_mails.iter(){
            if size > &features.size{
                return Err("max-size_limit-reached");
            }
        }
    }

    if features.start_tls{
        match connection{
            Connected::InSecure(_)=>{
                match io::secure_send_with_response(&mut connection,format!("STARTTLS\r\n")).await{
                    Ok(response)=>{
                        if response.result{
                            match connect::start_tls(connection, config.domain.clone(), &port).await{
                                Ok(v)=>{
                                    println!(">>>>>> start_tls complete");
                                    connection = v;
                                },
                                Err(_)=>{
                                    return Err("failed-start-tls");
                                }
                            }
                        }
                    },
                    Err(_e)=>{
                        println!("!!! failed-reply_hello : {:?}",_e);
                        return Err("failed-wait_for_hello");
                    }
                }
            },
            _=>{}
        }
    }

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
                println!("!!! send_mail_error : {:?}",_e);
                failed.push(tracking_id);
                match io::secure_send_with_response(&mut connection,"RESET\r\n".to_string()).await{
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

pub async fn process_mail(connection:&mut Connected,commands:&mut Vec<String>,features:&Features)->Result<(),&'static str>{

    // println!(">>> process_mail");

    loop{

        if commands.len() == 0{
            break;
        }

        let command = commands.remove(0);

        match io::secure_send(connection,command.clone()).await{
            Ok(_)=>{},
            Err(_)=>{
                // println!("!!! failed-email_command-send");
                return Err("connection_closed-write");
            }
        }

        // println!("{:?}",command);

        // println!(">>> command_sent");
        
        if !features.pipeline{
            match io::secure_read(connection).await{
                Ok(response)=>{
                    // println!(">>> response : {:?}",response);
                    if !response.result{
                        // println!(">>> response : {:?}",response);
                        return Err("smtp_error");
                    }
                },
                Err(_e)=>{
                    println!("!!! failed-non_pipeline-read : {:?}",_e);
                    return Err("connection_closed-read");
                }
            }
        }

    }

    // println!(">>> commands sent");

    if features.pipeline{
        let mut index = 0;
        loop{
            if index == 4{
                // println!("responses finished");
                break;
            }
            match io::secure_read_qued(connection).await{
                Ok(responses)=>{
                    for response in responses{
                        // println!(">>> response : {:?}",response);
                        if !response.result{
                            return Err("smtp_error");
                        } else {
                            index += 1;
                        }
                    }
                },
                Err(_)=>{
                    return Err("connection_closed-read_qued");
                }
            }
        }
    }

    return Ok(());

}