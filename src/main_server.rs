use tokio;
use crate::server::config::{ServerConfig,ProcessMail,CheckMail};
use json::JsonValue;
use flume::Sender as FlumeSender;
use flume::unbounded as FlumeChannel;

mod common;
mod server;
mod io;

#[tokio::main]
async fn main(){

    println!("string letterman server");

    start_async_server().await;

}

//---------------------------------------
//server
//---------------------------------------

async fn start_async_server(){

    let config:ServerConfig;
    match ServerConfig::new(
        vec![25,465,587,2525],
        String::from("silversender.com"),
        format!("../secret/end.cert"),
        format!("../secret/end.rsa"),
        // format!("../secret/smtp_gzbemail_xyz.cert"),
        // format!("../secret/smtp_gzbemail_xyz.private_key"),
        // format!("../secret/end.rsa"),
        100_000,
        vec![
            String::from("../letter_man_que/que/que_1.rustque")
        ],
        5_000_000,
        5_000_000,
        100,
        String::from("../letter_man_que/email_files/"),
        1,
        false,false,true
    ).await{
        Ok(v)=>{config = v;},
        Err(_)=>{
            return;
        }
    }

    println!(">>> starting server");

    let (check_mail_sender,_check_mail_receiver) = FlumeChannel();
    let (process_mail_sender,_process_mail_receiver) = FlumeChannel();

    match server::init(
        config,
        check_email,
        process_email,
        check_mail_sender,
        process_mail_sender
    ).await{
        Ok(_)=>{},
        Err(_e)=>{
            println!("!!! server down : {:?}",_e);
        }
    }

    println!("server closed");

}

async fn process_email(_i:ProcessMail,_sender:FlumeSender<JsonValue>) -> Result<(),()>{

    // println!("{:?}",_i);

    // println!("{:?}",_i.files);

    for i in _i.files.iter(){
        if false {
            match crate::io::delete_file(format!("../letter_man_que/email_files/{}",i)).await{
                Ok(_)=>{
                    // println!("file deleted");
                },
                Err(_)=>{
                    println!("file delete failed");
                }
            }
        }
    }

    Ok(())

}

async fn check_email(_i:CheckMail,_sender:FlumeSender<JsonValue>) -> Result<bool,()>{

    // println!("{:?}",_i);

    if true {
        if _i.to == "gzbakku2@localhost"{
            return Ok(false);
        }
    }

    Ok(true)

}

