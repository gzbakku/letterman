
mod io;
mod email;
mod connection;

pub use email::{Email,Action};
pub use connection::Connection;

///
///```
/// use std::time::Instant;
/// use letterman::client::{Connection,Email};
/// 
///async fn start_async_client() {
///
/// println!(">>> sending mail async");
///
/// //d://workstation/expo/rust/letterman/letterman/keys/private.key
///
/// let key:String;
/// match client::read_key("../secret/private.key".to_string()).await{
///     Ok(v)=>{
///         key = v;
///     },
///     Err(e)=>{
///         println!("!!! {:?}",e);
///         return;
///     }
/// }
///
/// // println!("{:?}",key);
///
/// let mut conn:Connection;
/// if SEND_TO_DKIMVALIDATOR{
///     match Connection::new(
///         String::from("dkimvalidator.com"),
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
/// } else if SEND_TO_LOCALHOST {
///     //localhost
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
/// } else {
///     return;
/// }
///
/// for i in 0..1{
///     conn.add(build_mail_new(i.to_string()));
/// }
///
/// // println!("mails added");
///
/// let hold = Instant::now();
///
/// if true{
///     match conn.send().await{
///         Ok(_v)=>{
///             println!("send successfull  : {:?} {:?}",_v.0,_v.1);
///         },
///         Err(_e)=>{
///             println!("send failed : {:?}",_e);
///         }
///     }
/// }
///
/// println!("finished in  : {:?}",hold.elapsed());
///
/// }
/// 
/// fn build_mail_new(tracking_id:String) -> client::Email{
///
/// let mut email = client::Email::new();
///
/// email.server_name(String::from("mailcenter.herokuapp.com"));
/// email.name(String::from("gzbakku"));
/// email.from(String::from("akku@silvergram.in"));
/// email.tracking_id(tracking_id);
///
/// if SEND_TO_DKIMVALIDATOR{
///     email.to(String::from("yPEHwb2zlYrYM9@dkimvalidator.com"));
///     email.receiver(String::from("yPEHwb2zlYrYM9@dkimvalidator.com"));
/// } else if SEND_TO_LOCALHOST {
///     email.to(String::from("gzbakku@localhost"));
///     email.receiver(String::from("gzbakku@localhost"));
/// }
///
/// email.subject(String::from("hello world"));
/// email.body(String::from("first message\r\nsecond message\r\nthird message"));
///
/// if 1 == 1 {
///     email.html(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"));
/// }
///
/// if 1 == 1 {
///     email.attach("d://workstation/expo/rust/letterman/letterman/drink.png".to_string());
/// }
///
/// if 1 == 1 {
///     let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
///     email.attach_base64("drink_1.png".to_string(),base64_data,"image/png".to_string());
/// }
///
/// return email;
///
/// }
/// 
/// 
///```

pub async fn read_key(path:String) -> Result<String,String>{
    match crate::io::read_as_text(path).await{
        Ok(v)=>{return Ok(v)},
        Err(e)=>{return Err(format!("failed-read_key => {}",e));}
    }
}

