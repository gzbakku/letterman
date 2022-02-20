use tokio;
// use crate::server::config::{ServerConfig,ProcessMail,CheckMail};
use crate::client::Connection;
use flume::unbounded as FlumeChannel;
use json::JsonValue;
use flume::Sender as FlumeSender;
use std::time::Instant;
use tokio::spawn as TokioSpawn;

mod common;
mod client;
mod io;

#[tokio::main]
async fn main(){

    println!("welcome to letterman gmail testing");

    start_async_client().await;

}

// const DOMAIN:&'static str = "dkimvalidator.com";
// const EMAIL:&'static str = "3f35apdz36lypl@dkimvalidator.com";

const DOMAIN:&'static str = "gmail.com";
const EMAIL:&'static str = "gzbakku@gmail.com";

//---------------------------------------
//client
//---------------------------------------

async fn start_async_client() {

    println!(">>> sending mail async");

    let mut conn:Connection;
    match Connection::new(
        // String::from("gmail.com"),
        DOMAIN.to_string(),
        String::from("smtp.gzbemail.xyz"),
        KEY.to_string(),
        String::from("dkim"),
        format!("gzbemail.xyz")
    ){
        Ok(v)=>{conn = v;},
        Err(_)=>{
            return;
        }
    }

    // println!("connection successfull");

    for i in 0..1{
        conn.add(build_mail_new(i.to_string()));
    }

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

    email.server_name(String::from("smtp.gzbemail.xyz"));
    email.name(String::from("akku"));
    email.from(String::from("akku@gzbemail.xyz"));
    email.tracking_id(tracking_id);

    // email.to(String::from("gzbakku@gmail.com"));
    // email.receiver(String::from("gzbakku@gmail.com"));

    email.to(EMAIL.to_string());
    email.receiver(EMAIL.to_string());
    
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