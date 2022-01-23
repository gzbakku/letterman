use tokio;
use crate::server::config::{ServerConfig,ProcessMail,CheckMail};
use crate::client::Connection;
use flume::unbounded as FlumeChannel;
use json::JsonValue;
use flume::Sender as FlumeSender;
use std::time::Instant;
use tokio::spawn as TokioSpawn;

mod common;
mod client;
mod server;
mod io;

// mod client_old;
// mod io;

const SEND_TO_DKIMVALIDATOR:bool = false;
const SEND_TO_LOCALHOST:bool = true;
const SEND_TO_ANYONE:bool = false;

#[tokio::main]
async fn main(){

    println!("welcome to letterman");

    let cmd_line = std::env::args();

    for cmd in cmd_line{
        if cmd.contains("--client"){
            let mut collect = Vec::new();
            for _ in 0..1{
                collect.push(TokioSpawn(async move {
                    start_async_client().await;
                }));
            }
            for i in collect{
                match i.await{
                    Ok(_)=>{},
                    Err(_)=>{}
                }
            }
        } else
        if cmd.contains("--server"){
            start_async_server().await;
        }
    }

}

//---------------------------------------
//client
//---------------------------------------

async fn start_async_client() {

    println!(">>> sending mail async");

    //d://workstation/expo/rust/letterman/letterman/keys/private.key

    let key:String;
    match client::read_key("../secret/private.key".to_string()).await{
        Ok(v)=>{
            key = v;
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return;
        }
    }

    // println!("{:?}",key);

    let mut conn:Connection;
    if SEND_TO_DKIMVALIDATOR{
        match Connection::new(
            String::from("dkimvalidator.com"),
            String::from("mailcenter.herokuapp.com"),
            key,
            String::from("dkim"),
            String::from("silvergram.in"),
        ){
            Ok(v)=>{conn = v;},
            Err(_)=>{
                return;
            }
        }
    } else if SEND_TO_LOCALHOST {
        //localhost
        match Connection::new(
            String::from("localhost"),
            String::from("mailcenter.herokuapp.com"),
            key,
            String::from("dkim"),
            String::from("silvergram.in"),
        ){
            Ok(v)=>{conn = v;},
            Err(_)=>{
                return;
            }
        }
    } else if SEND_TO_ANYONE {
        match Connection::new(
            String::from("xijih27584@veb34.com"),
            String::from("mailcenter.herokuapp.com"),
            key,
            String::from("dkim"),
            String::from("silvergram.in"),
        ){
            Ok(v)=>{conn = v;},
            Err(_)=>{
                return;
            }
        }
    } else {
        return;
    }

    for i in 0..1{
        conn.add(build_mail_new(i.to_string()));
    }

    // println!("mails added");

    let hold = Instant::now();

    if true{
        match conn.send().await{
            Ok(_v)=>{
                println!("send successfull => success : {:?} | failed : {:?}",_v.0.len(),_v.1);
            },
            Err(_e)=>{
                println!("send failed : {:?}",_e);
            }
        }
    }

    println!("finished in  : {:?}",hold.elapsed());

}

fn build_mail_new(tracking_id:String) -> client::Email{

    let mut email = client::Email::new();

    email.server_name(String::from("mailcenter.herokuapp.com"));
    email.name(String::from("gzbakku"));
    email.from(String::from("akku@silvergram.in"));
    email.tracking_id(tracking_id);
    // email.to(String::from("gzbakku@gmail.com"));

    if SEND_TO_DKIMVALIDATOR{
        email.to(String::from("yPEHwb2zlYrYM9@dkimvalidator.com"));
        email.receiver(String::from("yPEHwb2zlYrYM9@dkimvalidator.com"));
    } else if SEND_TO_LOCALHOST {
        email.to(String::from("gzbakku@localhost"));
        email.receiver(String::from("gzbakku1@localhost"));
        email.receiver(String::from("gzbakku2@localhost"));
    }
    
    email.subject(String::from("hello world"));
    email.body(String::from("first message\r\nsecond message\r\nthird message"));

    if 1 == 1 {
        email.html(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"));
    }

    if 1 == 0 {
        email.attach("d://workstation/expo/rust/letterman/letterman/drink.png".to_string());
    }

    if 1 == 0 {
        let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
        email.attach_base64("drink_1.png".to_string(),base64_data,"image/png".to_string());
    }

    return email;

}

//---------------------------------------
//server
//---------------------------------------

async fn start_async_server(){

    let config:ServerConfig;
    match ServerConfig::new(
        vec![587,2525],
        String::from("silversender.com"),
        format!("../secret/end.cert"),
        format!("../secret/end.rsa"),
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

//---------------------------------------
//old client
//---------------------------------------

// async fn start_client_old() {
//     println!(">>> sending mail old async");
//     let email = build_mail_old();
//     match email.send_tokio().await{
//         Ok(_)=>{
//             println!("email sent");
//         },
//         Err(e)=>{
//             println!("email failed : {:?}",e);
//         }
//     }
// }

// fn build_mail_old() -> client_old::Email{

//     let mut email = client_old::Email::new();

//     match client_old::read_key("d://workstation/expo/rust/letterman/letterman/keys/private.key".to_string()){
//         Ok(key)=>{
//             email.private_key(key);
//         },
//         Err(e)=>{
//             println!("!!! {:?}",e);
//         }
//     }

//     email.dkim_selector(String::from("dkim"));
//     email.server_name(String::from("mailcenter.herokuapp.com"));
//     email.name(String::from("gzbakku"));
//     email.from(String::from("akku@silvergram.in"));
//     // email.to(String::from("gzbakku@gmail.com"));
//     email.to(String::from("gzbakku@localhost"));
//     // email.to(String::from("aGvnJQ7c8tXREw@dkimvalidator.com"));
//     email.subject(String::from("hello world"));
//     email.body(String::from("first message\r\nsecond message\r\nthird message"));

//     if 1 == 0 {
//         email.body(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"));
//         email.is_html();
//     }

//     if 1 == 1 {
//         email.attach("d://workstation/expo/rust/letterman/letterman/drink.png".to_string());
//     }

//     if false {
//         let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
//         email.attach_base64("drink.png".to_string(),base64_data);
//     }

//     return email;

// }

