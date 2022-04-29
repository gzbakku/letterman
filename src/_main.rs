use tokio;
// use crate::server::config::{ServerConfig,ProcessMail,CheckMail};
use crate::client::Connection;
// use flume::unbounded as FlumeChannel;
// use json::JsonValue;
// use flume::Sender as FlumeSender;
use std::time::Instant;
// use tokio::spawn as TokioSpawn;

mod common;
mod client;
mod io;

#[tokio::main]
async fn main(){

    println!("welcome to letterman gmail testing");

    start_async_client().await;

}

// const DOMAIN:&'static str = "dkimvalidator.com";
// const EMAIL:&'static str = "6pwmslelhdy3rr@dkimvalidator.com";
// const GE_KEY:&'static str = "../secret/ge_dkim_private_key.txt";

const DOMAIN:&'static str = "gmail.com";
const EMAIL:&'static str = "gzbakku@gmail.com";
const GE_KEY:&'static str = "./ge_dkim_private_key.txt";

//---------------------------------------
//client
//---------------------------------------

async fn start_async_client() {

    let key:String;
    match client::read_key(GE_KEY.to_string()).await{
        Ok(v)=>{
            key = v;
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return;
        }
    }

    println!(">>> sending mail async");

    let mut conn:Connection;
    match Connection::new(
        // String::from("gmail.com"),
        DOMAIN.to_string(),
        String::from("smtp.gzbemail.xyz"),
        key,
        String::from("dkim"),
        format!("gzbemail.xyz")
    ){
        Ok(v)=>{conn = v;},
        Err(_)=>{
            return;
        }
    }

    // println!("connection successfull");

    for i in 0..(1 as u8){
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

    if 1 == 0 {
        email.html(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world with line</h1>\r\n </body> </html> &#169; https://google.com"));
    }

    if 1 == 1 {
        email.html(HTML.to_string());
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

const _HTML:&'static str = r#"
<html>
    <head>
        <title>GzbEmail</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
            @import url('https://fonts.googleapis.com/css2?family=Nunito:wght@200&family=Roboto:wght@100;300&display=swap');
            * {
                font-family:"Nunito";
            }
            .logo{
                padding: 10px;
                height:150px;
                /* border:5px solid green; */
            }
            .logo-img{
                display:inline-block;
                height:80px;
                width: auto;
                /* border:5px solid blue; */
            }
            .logo-text{
                padding-top: 25px;
                font-size: 32px;
                vertical-align: top;
                display:inline-block;
                /* border:5px solid red; */
                color:#d0cdff;
                font-weight: bold;
            }
            .footer{
                font-weight: bold;
                text-align: center;
                margin-top: 100px;
            }
            .ud-msg{
                padding: 10px;
                font-size: 24px;
            }
                .ud-msg-center{
                    /* text-align: center; */
                }
            .otp{
                margin-top: 50px;
                max-width: 500px;
                margin: auto;
                /* border:5px solid red; */
                display: grid;
                grid-template-columns: 300px auto;
                text-align: center;
            }
                .otp-tag{
                    color:#ed005f;
                    padding: 10px;
                    font-size: 24px;
                    word-break: break-all;
                    /* border:5px solid green; */
                    font-weight: bold;
                }
                .otp-val{
                    padding: 10px;
                    font-size: 24px;
                    word-break: break-all;
                    /* border:5px solid purple; */
                    font-weight: bold;
                }

            

            .links{
                
            }
                .link{
                    background-color: #ed005f;
                    padding: 10px;
                    border-radius: 10px;
                    margin: 10px;
                    display: inline-block;
                    color:white;
                    font-size: 24px;
                    font-weight: bold;
                    text-decoration: none;
                }

            .heading{
                padding:10px;
            }

            @media only screen and (max-width: 600px) {
                .otp {
                    width:100%;
                    display: block;
                }
            }
        </style>
    </head>
    <body>
        <div class="logo">
            <img src="https://gzbemail.xyz/assets/images/logo.png" class="logo-img" alt="">
            <div class="logo-text">GZBEMAIL</div>
        </div>
            <h1 class="heading">Some Heading</h1>
            <div class="ud-msg ud-msg-center">
                Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.
            </div>
            <div class="otp">
                <div class="otp-tag">Account Activation Otp</div>
                <div class="otp-val">356987</div>
            </div>
            <div class="links">
                <a class="link" href="https://gzbemail.xyz">some link</a>
            </div> 
        {add}
        <div class="footer">
            &#169; GzbAkku
        </div>
    </body>
</html>
"#;

//@import url('https://fonts.googleapis.com/css2?family=Nunito:wght@200&family=Roboto:wght@100;300&display=swap');

const HTML:&'static str = r#"
<html>
    <head>
        <title>GzbEmail</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
            @import url('https://fonts.googleapis.com/css2?family=Nunito:wght@200&family=Roboto:wght@100;300&display=swap');
            * {
                font-family:"Nunito";
            }
            .logo{
                padding: 10px;
                height:150px;
                /* border:5px solid green; */
            }
            .logo-img{
                display:inline-block;
                height:80px;
                width: auto;
                /* border:5px solid blue; */
            }
            .logo-text{
                padding-top: 25px;
                font-size: 32px;
                vertical-align: top;
                display:inline-block;
                /* border:5px solid red; */
                color:#d0cdff;
                font-weight: bold;
            }
            .footer{
                font-weight: bold;
                text-align: center;
                margin-top: 100px;
            }
            .ud-msg{
                padding: 10px;
                font-size: 24px;
            }
                .ud-msg-center{
                    /* text-align: center; */
                }
            .otp{
                margin-top: 50px;
                max-width: 500px;
                margin: auto;
                /* border:5px solid red; */
                display: grid;
                grid-template-columns: 300px auto;
                text-align: center;
            }
                .otp-tag{
                    color:#ed005f;
                    padding: 10px;
                    font-size: 24px;
                    word-break: break-all;
                    /* border:5px solid green; */
                    font-weight: bold;
                }
                .otp-val{
                    padding: 10px;
                    font-size: 24px;
                    word-break: break-all;
                    /* border:5px solid purple; */
                    font-weight: bold;
                }

            

            .links{
                
            }
                .link{
                    background-color: #ed005f;
                    padding: 10px;
                    border-radius: 10px;
                    margin: 10px;
                    display: inline-block;
                    color:white;
                    font-size: 24px;
                    font-weight: bold;
                    text-decoration: none;
                }

            .heading{
                padding:10px;
            }

            @media only screen and (max-width: 600px) {
                .otp {
                    width:100%;
                    display: block;
                }
            }
        </style>
    </head>
    <body>
        <div class="logo">
            <img src="https://gzbemail.xyz/assets/images/logo.png" class="logo-img" alt="">
            <div class="logo-text">GZBEMAIL</div>
        </div>
            <h1 class="heading">Some Heading</h1>
            <div class="ud-msg ud-msg-center">
            😃 Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.
            </div>
            <div class="otp">
                <div class="otp-tag">Account Activation Otp</div>
                <div class="otp-val">356987</div>
            </div>
            <div class="links">
                <a class="link" href="https://gzbemail.xyz">some link</a>
            </div> 
        {add}
        <div class="footer">
            &#169; GzbAkku
        </div>
    </body>
</html>
"#;

/*

Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.

*/