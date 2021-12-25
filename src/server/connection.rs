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
use crate::server::{BAD_AUTH,DOWN,OK_REPLY,BAD_SEQUENCE,USER_NOT_FOUND,DATA_START,QUIT_REPLY};
use crate::server::jsonify::init as jasonify_init;
use letterman_email_body_parser::EmailBody;
use flume::Sender as FlumeSender;
use json::JsonValue;

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

        let hold_check_mail_sender = check_mail_sender.clone();

        //from message
        if email.status == 0{
            match io::async_server_read_till(
                &mut stream,
                &info_lock.message_end,
                &mut buffer,max_size_local
            ).await{
                Ok(request_string)=>{
                    if request_string.contains("MAIL FROM"){
                        // println!("from : {:?}",request_string);
                        match info_lock.regex.email.captures(&request_string){
                            Some(captures)=>{
                                if info_lock.validate_spf{
                                    match captures.get(2){
                                        Some(v)=>{
                                            //check if host can even send this email
                                            match TokioSpfCheck(
                                                &info_lock.spf_config,
                                                ip.clone(),
                                                &server_name,
                                                &v.as_str().to_string()
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
                                            match send_message(&mut stream, BAD_SEQUENCE).await{
                                                Ok(_)=>{},Err(_)=>{break;}
                                            }
                                        }
                                    }//get email domain match from regex
                                }
                                match captures.get(0){
                                    Some(v)=>{
                                        email.reset();
                                        email.from = v.as_str().to_string();
                                        email.status = 1;
                                        match send_message(&mut stream, OK_REPLY).await{
                                            Ok(_)=>{},Err(_)=>{break;}
                                        }
                                    },
                                    None=>{
                                        email.reset();
                                        match send_message(&mut stream, BAD_SEQUENCE).await{
                                            Ok(_)=>{},Err(_)=>{break;}
                                        }
                                    }
                                }//get full email match from regex
                            },
                            None=>{
                                email.reset();
                                match send_message(&mut stream, BAD_SEQUENCE).await{
                                    Ok(_)=>{},Err(_)=>{break;}
                                }
                            }//no email regex found in requets stirng
                        }
                    } else if request_string.contains("RSET"){
                        email.reset();
                        match send_message(&mut stream, OK_REPLY).await{
                            Ok(_)=>{},Err(_)=>{break;}
                        }
                    } else if request_string.contains("QUIT"){
                        match send_message(&mut stream, QUIT_REPLY).await{
                            Ok(_)=>{},Err(_)=>{}
                        }
                        break;
                    } else {
                        email.reset();
                        match send_message(&mut stream, BAD_SEQUENCE).await{
                            Ok(_)=>{},Err(_)=>{break;}
                        }
                    }//no request string match found
                },
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }//read failed on connection
            }
        }//rcpt to request

        //to message
        if email.status == 1{
            match io::async_server_read_till(
                &mut stream,
                &info_lock.message_end,
                &mut buffer,max_size_local
            ).await{
                Ok(request_string)=>{
                    if request_string.contains("RCPT TO"){
                        // println!("to : {:?}",request_string);
                        match info_lock.regex.email.captures(&request_string){
                            Some(captures)=>{
                                match captures.get(0){
                                    Some(v)=>{
                                        match check_email(CheckMail{
                                            from:email.from.clone(),
                                            to:email.to.clone()
                                        },hold_check_mail_sender).await{
                                            Ok(result)=>{
                                                if result == true{
                                                    match send_message(&mut stream, OK_REPLY).await{
                                                        Ok(_)=>{},Err(_)=>{break;}
                                                    }
                                                    email.to = v.as_str().to_string();
                                                    email.status = 2;
                                                } else {
                                                    email.reset();
                                                    match send_message(&mut stream, USER_NOT_FOUND).await{
                                                        Ok(_)=>{},Err(_)=>{break;}
                                                    }
                                                }
                                            },
                                            Err(_)=>{
                                                email.reset();
                                                match send_message(&mut stream, BAD_SEQUENCE).await{
                                                    Ok(_)=>{},Err(_)=>{break;}
                                                }
                                            }
                                        }
                                    },
                                    None=>{
                                        email.reset();
                                        match send_message(&mut stream, BAD_SEQUENCE).await{
                                            Ok(_)=>{},Err(_)=>{break;}
                                        }
                                    }//get matching email from captures
                                }
                            },
                            None=>{
                                email.reset();
                                match send_message(&mut stream, BAD_SEQUENCE).await{
                                    Ok(_)=>{},Err(_)=>{break;}
                                }
                            }//no email captured in regex
                        }
                    } else {
                        match send_message(&mut stream, BAD_SEQUENCE).await{
                            Ok(_)=>{},Err(_)=>{break;}
                        }
                    }//no requets string match found
                },
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }
            }//reader
        }//email to request

        //data message
        if email.status == 2{
            match io::async_server_read_till(
                &mut stream,
                &info_lock.message_end,
                &mut buffer,max_size_local
            ).await{
                Ok(request_string)=>{
                    if request_string.contains("DATA"){
                        // println!("data : {:?}",request_string);
                        match send_message(&mut stream, DATA_START).await{
                            Ok(_)=>{
                                email.status = 3;
                            },Err(_)=>{break;}
                        }
                    } else {
                        match send_message(&mut stream, BAD_SEQUENCE).await{
                            Ok(_)=>{},Err(_)=>{break;}
                        }
                    }//no requets string match found
                },
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }
            }//reader
        }//email to request

        //data message
        //from message
        if email.status == 3{
            // println!("where");
            match io::async_server_read_till(
                &mut stream,
                &info_lock.data_end,
                &mut buffer,max_size_local
            ).await{
                Ok(request_string)=>{
                    // println!(">>> parsing email");
                    let hold:Vec<&str> = request_string.split("\r\n").collect();
                    match EmailBodyParser(hold,&info_lock.email_body_parser_config){
                        Ok(mut email_body)=>{
                            if info_lock.validate_dkim{//if dkim validation is turned on
                                match email_body.validate(&info_lock.email_body_parser_config).await{
                                    Ok(_)=>{
                                        match process_mail(
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
                            match send_message(&mut stream, BAD_SEQUENCE).await{
                                Ok(_)=>{},Err(_)=>{break;}
                            }
                        }
                    }//parse email body 

                },//found \r\n.\r\n
                Err(_)=>{
                    println!("failed async_server_read_till");
                    error(stream,DOWN).await;
                    break;
                }
            }//read email body
        }//data input message

    }

}

async fn process_mail(
    stream:&mut TokioRustLsServerTlsStream<TcpStream>,
    email:Email,
    body:EmailBody,
    que_sender:&mut Sender<QueMessage>,
    attachment_storage_dir:&String
)->Result<(),()>{
    match jasonify_init(email,body,attachment_storage_dir).await{
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