use dns_lookup::lookup_addr;
use std::sync::Arc;
use std::net::SocketAddr;
use std::future::Future;
use std::net::IpAddr;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream as TokioRustLsServerTlsStream;
use tokio_spf_validator::check as TokioSpfCheck;
use letterman_email_body_parser::init as EmailBodyParser;
use crate::server::config::{ServerInfo};
use crate::server::io;
use crate::server::config::{QueMessage,CheckMail,Email,QueAddMessage,Signal};
use crate::server::{
    BAD_AUTH,DOWN,OK_REPLY,BAD_SEQUENCE,USER_NOT_FOUND,
    DATA_START,QUIT_REPLY,NO_RECEIVER_FOUND,UNHANDLED,BAD_PARAMS
};
use crate::server::jsonify::init as jasonify_init;
use letterman_email_body_parser::EmailBody;
use flume::Sender as FlumeSender;
use json::JsonValue;

const DATA_END:&'static str = "\r\n.\r\n";

pub async fn error(stream:TokioRustLsServerTlsStream<TcpStream>,e:&'static str){
    let mut stream = stream;
    match io::send_server(&mut stream,&format!("{}",e)).await{
        Ok(_)=>{},
        Err(_)=>{}
    }
}

pub async fn send_message(stream:&mut TokioRustLsServerTlsStream<TcpStream>,e:&'static str)->Result<(),()>{
    let mut stream = stream;
    match io::send_server(&mut stream,&format!("{}",e)).await{
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err(());
        }
    }
}

pub async fn handle<F,T>(
    stream:TokioRustLsServerTlsStream<TcpStream>,
    addr:SocketAddr,
    info:Arc<RwLock<ServerInfo>>,
    check_email:F,
    ip:IpAddr,
    que_sender:Sender<QueMessage>,
    check_mail_sender:FlumeSender<JsonValue>
)
where
    F:Fn(CheckMail,FlumeSender<JsonValue>) -> T + Unpin + Send + 'static + Copy + Sync,
    T:Future<Output = Result<bool,()>> + Send + 'static,
{

    let mut stream = stream;
    let mut que_sender = que_sender;
    let info_lock = info.read().await;

    // println!("### connection started");

    //------------------------------
    //say hello
    //------------------------------

    //say helo
    match io::send_server(&mut stream,&format!("220 {} ESMTP\r\n",&info_lock.name)).await{
        Ok(_)=>{},
        Err(_)=>{
            return error(stream,DOWN).await;
        }
    }

    // println!("### helo sent");

    //get ehlo response
    let server_name:String;
    match io::read_server(&mut stream,&info_lock.regex.ehlo).await{
        Ok(mut response)=>{
            if response.len() != 2{return;}
            server_name = response.remove(1);
        },
        Err(_)=>{
            return error(stream,DOWN).await;
        }
    }

    //------------------------------
    //validate rdns
    //------------------------------

    //verify servername with rdns records
    let hostname:String;
    match lookup_addr(&addr.ip()){
        Ok(v)=>{
            hostname = v;
        },
        Err(_)=>{
            return error(stream,DOWN).await;
        }
    }

    //check if hostname and servername match
    if info_lock.validate_rdns{
        if hostname != server_name{
            return error(stream,BAD_AUTH).await;
        }
    }

    //------------------------------
    //send features
    //------------------------------

    //send features
    let mut features = String::new();
    // features.push_str(&format!("250-{} Hello {}\r\n",&info_lock.name,&hostname));
    features.push_str(&format!("250-SIZE {}\r\n",&info_lock.size));
    features.push_str(&format!("250-PIPELINING\r\n"));
    features.push_str(&format!("250-8BITMIME\r\n"));
    match io::send_server(&mut stream,&features).await{
        Ok(_)=>{},
        Err(_)=>{
            return error(stream,DOWN).await;
        }
    }

    //------------------------------
    //process further queries
    //------------------------------

    let mut email = Email::new();
    let mut buffer = Vec::new();
    let attachment_storage_dir = &info_lock.attachment_storage_dir.clone();
    let max_size_local = info_lock.size as usize;

    loop{

        if email.status == 3{email.status = 4;}
        let hold_check_mail_sender = check_mail_sender.clone();

        //read message
        let message:String;
        if email.status == 4{
            match io::read_server_till_end(
                &mut stream,
                &mut buffer,
                &DATA_END
            ).await{
                Ok(request_string)=>{
                    message = request_string;
                },
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }//read failed on connection
            }
        } else {
            match io::async_server_read_till(
                &mut stream,
                &info_lock.message_end,
                &mut buffer,max_size_local
            ).await{
                Ok(request_string)=>{
                    message = request_string;
                },
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }//read failed on connection
            }
        }

        if email.status < 3{
            if message.contains("MAIL FROM:") || message.contains("MAIL FROM :"){
                // println!("MAIL FROM");
                email.reset();
                let split_message:Vec<&str> = message.split(":").collect();
                match info_lock.regex.email.captures(&split_message[1]){
                    Some(captures)=>{
                        if info_lock.validate_spf{
                            match captures.get(3){
                                Some(v)=>{
                                    //check if host can even send this email
                                    let domain = v.as_str().to_string();
                                    match TokioSpfCheck(
                                        &info_lock.spf_config,
                                        ip.clone(),
                                        &server_name,
                                        &domain
                                    ).await{
                                        Ok(query_result)=>{
                                            email.spf = query_result;
                                        },
                                        Err(_)=>{
                                            email.reset();
                                            match send_message(&mut stream, DOWN).await{
                                                Ok(_)=>{},Err(_)=>{break;}
                                            }
                                        }
                                    }
                                },
                                None=>{
                                    email.reset();
                                    match send_message(&mut stream, BAD_PARAMS).await{
                                        Ok(_)=>{},Err(_)=>{break;}
                                    }
                                }
                            }//get email domain match from regex
                        }
                        match captures.get(1){
                            Some(v)=>{
                                email.reset();
                                email.sender = v.as_str().to_string();
                                email.status = 1;
                                match send_message(&mut stream, OK_REPLY).await{
                                    Ok(_)=>{},Err(_)=>{break;}
                                }
                            },
                            None=>{
                                email.reset();
                                match send_message(&mut stream, BAD_PARAMS).await{
                                    Ok(_)=>{},Err(_)=>{break;}
                                }
                            }
                        }//get full email match from regex
                    },
                    None=>{
                        email.reset();
                        match send_message(&mut stream, BAD_PARAMS).await{
                            Ok(_)=>{},Err(_)=>{break;}
                        }
                    }//no email regex found in requets stirng
                }
            } else if message.contains("RCPT TO:") || message.contains("RCPT TO :"){
                // println!("RCPT TO");
                if email.sender.len() == 0{
                    email.reset();
                    match send_message(&mut stream, BAD_SEQUENCE).await{
                        Ok(_)=>{email.status = 0;},Err(_)=>{break;}
                    }
                } else {
                    let split_message:Vec<&str> = message.split(":").collect();
                    match info_lock.regex.email.captures(&split_message[1]){
                        Some(captures)=>{
                            match captures.get(1){
                                Some(v)=>{
                                    let to = v.as_str().to_string();
                                    match check_email(CheckMail{
                                        from:email.sender.clone(),
                                        to:to.clone()
                                    },hold_check_mail_sender).await{
                                        Ok(result)=>{
                                            if result == true{
                                                match send_message(&mut stream, OK_REPLY).await{
                                                    Ok(_)=>{},Err(_)=>{break;}
                                                }
                                                email.receivers.push(to); 
                                                email.status = 2;
                                            } else {
                                                // email.reset();
                                                match send_message(&mut stream, USER_NOT_FOUND).await{
                                                    Ok(_)=>{},Err(_)=>{break;}
                                                }
                                            }
                                        },
                                        Err(_)=>{
                                            email.reset();
                                            match send_message(&mut stream, DOWN).await{
                                                Ok(_)=>{},Err(_)=>{break;}
                                            }
                                        }
                                    }
                                },
                                None=>{
                                    email.reset();
                                    match send_message(&mut stream, BAD_PARAMS).await{
                                        Ok(_)=>{},Err(_)=>{break;}
                                    }
                                }//get matching email from captures
                            }
                        },
                        None=>{
                            email.reset();
                            match send_message(&mut stream, BAD_PARAMS).await{
                                Ok(_)=>{},Err(_)=>{break;}
                            }
                        }//no email captured in regex
                    }
                }
            } else if message.contains("DATA"){
                // println!("DATA");
                if !email.check(){
                    email.reset();
                    match send_message(&mut stream, BAD_SEQUENCE).await{
                        Ok(_)=>{email.status = 0;},Err(_)=>{break;}
                    }
                } else {
                    if email.receivers.len() == 0{
                        email.reset();
                        match send_message(&mut stream, NO_RECEIVER_FOUND).await{
                            Ok(_)=>{email.status = 0;},Err(_)=>{break;}
                        }
                    } else {
                        match send_message(&mut stream, DATA_START).await{
                            Ok(_)=>{email.status = 3;},Err(_)=>{break;}
                        }
                    }
                }
            } else if message.contains("RSET"){
                // println!("RSET");
                email.reset();
                match send_message(&mut stream, OK_REPLY).await{
                    Ok(_)=>{},Err(_)=>{break;}
                }
            } else if message.contains("QUIT"){
                // println!("QUIT");
                match send_message(&mut stream, QUIT_REPLY).await{
                    Ok(_)=>{},Err(_)=>{}
                }
                break;
            } else {
                // println!("UNHANDLED : {:?}",message);
                email.reset();
                match send_message(&mut stream, UNHANDLED).await{
                    Ok(_)=>{},Err(_)=>{break;}
                }
            }
        }//check messages till data

        if email.status == 4{
            // println!("BODY");
            // println!("{:?}",message);
            let hold:Vec<&str> = message.split("\r\n").collect();
            // if hold[hold.len()-1].len() == 0{
            //     hold.remove(hold.len()-1);
            // }
            // println!("\n{:?}\n",hold);
            match EmailBodyParser(hold,&info_lock.email_body_parser_config){
                Ok(mut email_body)=>{
                    if info_lock.validate_dkim{//if dkim validation is turned on
                        match email_body.validate(&info_lock.email_body_parser_config).await{
                            Ok(_)=>{
                                match process_mail(
                                    // &info,
                                    &mut stream,
                                    email.flush(),
                                    email_body,
                                    &mut que_sender,
                                    &attachment_storage_dir
                                ).await{
                                    Ok(_)=>{
                                        email.reset();
                                    },
                                    Err(_)=>{break;}
                                }
                            },
                            Err(_)=>{//email dkim validation failed
                                match send_message(&mut stream, BAD_AUTH).await{
                                    Ok(_)=>{},Err(_)=>{break;}
                                }
                            }
                        }//validate dkim 
                    } else {//if dkim validation is turned off
                        match process_mail(
                            // &info,
                            &mut stream,
                            email.flush(),
                            email_body,
                            &mut que_sender,
                            &attachment_storage_dir
                        ).await{
                            Ok(_)=>{
                                email.reset();
                            },
                            Err(_)=>{break;}
                        }
                    }//if dkim validation is turned off
                },//email body parsed
                Err(_)=>{
                    match send_message(&mut stream, BAD_PARAMS).await{
                        Ok(_)=>{},Err(_)=>{break;}
                    }
                }
            }//parse email body 
            email.status = 0;
        }

    }//loop message

}

async fn process_mail(
    // info:&Arc<RwLock<ServerInfo>>,
    stream:&mut TokioRustLsServerTlsStream<TcpStream>,
    email:Email,
    body:EmailBody,
    que_sender:&mut Sender<QueMessage>,
    attachment_storage_dir:&String
)->Result<(),()>{
    match jasonify_init(
        // info,
        email,
        body,
        attachment_storage_dir
    ).await{
        Ok(body)=>{

            let (signal,sleeper) = Signal::new();
            match que_sender.send(
                QueMessage::Add(
                    QueAddMessage{
                        signal:signal.clone(),
                        body:body
                    }
                )
            ).await{
                Ok(_)=>{
                    // println!("!!! send que message success");
                    Signal::wait(sleeper).await;
                    if !Signal::check(signal).await {
                        match send_message(stream, DOWN).await{
                            Ok(_)=>{},Err(_)=>{return Err(());}
                        }
                    } else {
                        match send_message(stream, OK_REPLY).await{
                            Ok(_)=>{},Err(_)=>{return Err(());}
                        }
                    }
                },
                Err(_)=>{
                    // println!("!!! send que message failed");
                    match send_message(stream, DOWN).await{
                        Ok(_)=>{},Err(_)=>{return Err(());}
                    }
                }
            }

        },
        Err(_e)=>{//failed jsonify email
            // println!("!!! jsonify failed : {:?}",_e);
            match send_message(stream, DOWN).await{
                Ok(_)=>{},Err(_)=>{return Err(());}
            }
        }
    }//jsonify email
    return Ok(());
}