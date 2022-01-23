


//! Letterman is fast a Smpt server and client library with advanced smtp extanstion support.


mod common;
mod io;

///Smtp client provides smtp pipeline, spf validation, dkim validation and rdns extensions.Multiple emails can be sent over one connection but the domain for sender must be the same on each connection type.
///```
/// use std::time::Instant;
/// use letterman::client::{Connection,Email,read_key};
/// 
///
/// async fn start_async_client() {
///
///     println!(">>> sending mail async");
///
///     let key:String;
///     match client::read_key("../secret/private.key".to_string()).await{
///         Ok(v)=>{
///             key = v;
///         },
///         Err(e)=>{
///             println!("!!! {:?}",e);
///             return;
///         }
///     }
///
///     let mut conn:Connection;
///     match Connection::new(
///         String::from("localhost"),
///         String::from("mailcenter.herokuapp.com"),
///         key,
///         String::from("dkim"),
///         String::from("silvergram.in"),
///     ){
///         Ok(v)=>{conn = v;},
///         Err(_)=>{
///             return;
///         }
///     }
///
///     for i in 0..1{
///         conn.add(build_mail_new(i.to_string()));
///     }
///
///     let hold = Instant::now();
///
///     if true{
///         match conn.send().await{
///             Ok(_v)=>{
///                 println!("send successfull  : {:?} {:?}",_v.0,_v.1);
///             },
///             Err(_e)=>{
///                 println!("send failed : {:?}",_e);
///             }
///         }
///     }
///
///     println!("finished in  : {:?}",hold.elapsed());
///
/// }
///
/// fn build_mail_new(tracking_id:String) -> client::Email{
///
///     let mut email = client::Email::new();
///
///     email.server_name(String::from("mailcenter.herokuapp.com"));
///     email.name(String::from("gzbakku"));
///     email.from(String::from("akku@silvergram.in"));
///     email.tracking_id(tracking_id);
///     email.to(String::from("gzbakku@localhost"));
///     //the receivers will receive the message this feature allows cc and bcc smtp functions
///     email.receiver(String::from("gzbakku@localhost"));
///     email.receiver(String::from("gzbakku1@localhost"));
///     email.subject(String::from("hello world"));
///     email.body(String::from("first message\r\nsecond message\r\nthird message"));
///
///     if 1 == 1 {
///         email.html(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"));
///     }
///
///     if 1 == 1 {
///         email.attach("d://workstation/expo/rust/letterman/letterman/drink.png".to_string());
///     }
///
///     if 1 == 1 {
///         let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
///         email.attach_base64("drink_1.png".to_string(),base64_data,"image/png".to_string());
///     }
///
///     return email;
///
/// }
///
///```
pub mod client;

///smtp server supports pipelining, rdns valiadationm,dkim valiadation and spf validation with mime type support, alternative body type support, quoted printable and base64 decoding support and 8bit mime support.
/// 
/// ```
/// use letterman::flume::unbounded as FlumeChannel;
/// use letterman::flume::Sender as FlumeSender;
/// use letterman::json::JsonValue;
/// use letterman::server::config::{ServerConfig};
/// use letterman::server::{init,CheckMail,ProcessMail};
/// 
/// async fn start_async_server(){
///
///     let config:ServerConfig;
///     match ServerConfig::new(
///         vec![587,2525],
///         String::from("silversender.com"),
///         format!("../secret/end.cert"),
///         format!("../secret/end.rsa"),
///         100_000,
///         String::from("../letter_man_que/que/que.akku"),
///         5_000_000,
///         5,
///         String::from("../letter_man_que/email_files/"),
///         1,
///         false,false,true
///     ).await{
///         Ok(v)=>{config = v;},
///         Err(_)=>{
///             return;
///         }
///     }
///
///     println!(">>> starting server");
///
///     let (check_mail_sender,_check_mail_receiver) = FlumeChannel();
///     let (process_mail_sender,_process_mail_receiver) = FlumeChannel();
///
///     match init(
///         config,
///         check_email,
///         process_email,
///         check_mail_sender,
///         process_mail_sender
///     ).await{
///         Ok(_)=>{},
///         Err(_e)=>{
///             println!("!!! server down : {:?}",_e);
///         }
///     }
///
///     println!("server closed");
///
/// }
/// 
/// async fn process_email(_i:ProcessMail,_sender:FlumeSender<JsonValue>) -> Result<(),()>{
///
///     for i in _i.files.iter(){
///         if false {
///             match letterman::io::delete_file(format!("../letter_man_que/email_files/{}",i)).await{
///                 Ok(_)=>{
///                     // println!("file deleted");
///                 },
///                 Err(_)=>{
///                     println!("file delete failed");
///                 }
///             }
///         }
///     }
///
///     Ok(())
///
/// }
///
/// async fn check_email(_i:CheckMail,_sender:FlumeSender<JsonValue>) -> Result<bool,()>{
///
///     Ok(true)
///
/// }
/// ```
/// 
pub mod server;

pub use flume;
pub use json;
pub use rustque;
