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

const KEY:&'static str = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEApwYXlQX8D2E/cgjKW5uLFvdY2NdjAm5MxbEpxuyty+/pmo9Q
zLBPN4y40yUW3aENPpRTSYaOa8QayrKO/6xzFoikFoFq75euqBdnZk4V5348Jer+
WGTsEKAGmAW0GIEyr+eLA7Q6obryOeGuQWtY1WzRnv5jTX2BJ+iJ+8t1//7EH6CC
1q7e9711rE/rGlTDkOnyC8pa1qM7IbHQbup+P3Z6CqOl0vXnyLwhold+pXIstYuq
xR7zJz3hJEnthRAyz1mDntXLSD7hXkv4zpQeZJcu4cq2x8qKoX/0mF0Ewbj46Il4
D8+5D1f/gx8iSSO2k742qs0I/wWv/atlQdxxYQIDAQABAoIBAHlWsK0fwuV9dbUn
c7Mnhd1yZkZp+1DZxsZcFYihiwU8Ts5tTXrWn0Yw4ljqefkatl9/LB6fNTpPJlOZ
cs+FfbqiG+sJrCFRZZ5SGlk1Yy5hA9tcI9kdwG23g/LPOe4Pdj5ajSBsv6edA7pP
HOQD026Bdqv2DThPdBQFGLEnN5t4UpPGCjV5rVRmln15dJoj5M3xKB38MaiknwhR
76kN4p6EtMzddUkHphuAm4iSSyD3lDTBXSlJn/N3XsLPsJ9jNKdPbFevceiuAyZW
NeJuuuOSUMoA7SFebwhXyXd4D/BcPu+9U1njwjrddxs4G/bAyjOgCj1mpjPUyb8b
0HsRVy0CgYEA0Y9p83FRmkvDs2p5CK851VeAX0b3eLVhifsH83Sj+BqJiroJgWjY
Dnow8zqnjKeAKa1P+G1NH3UM+HN3nw0c75MVet1/AV/fiJTwMKXBzUv7ULN9R+lo
Cxuk0pQ2mb+TunTe6dmxfkMTOcZV5bcAks0i/TqxlBniQUV6kXynh48CgYEAzAmK
LTHX6UTwOIQ+mAaQqeU4kV690YmjFI9EHbXV02RhzjuoXQUd+idN9af626TmYtNL
YazXKNgjuTpOgAFu78s0gI67kf4YeCXvqsxO++s8xnutfwe0CWuXO/rcfns0oY2/
ioWM8ekbTOuFYQhL0Pl8moVg/5F+ydevhol8gA8CgYEAuu19af0L+GFa2QDBpACB
yw75YIOyHcdVkToOFplV5eruA4P9FKVMDGXsohoQ+MM07Hg/XG9LGyNTBZAiBQsy
Y3XE9Er9jmFRyMhqFErXO+Rp8cuZlfrapwXqmThCGBUulmTHrrtTuzfjv2ZumrEJ
3ukDK/UeD+iizOxH79zMp3sCgYB+QRTwfFw3KwJeZm55Ee7oQj+rrG1WwI1aBoDG
xaLHiOEAhWfcD3OKPFIARW50BWjOFCMcEKCe1IfecRbsHHbyCNK3DhtA7nNjvU29
aWkid2CHDTbBWRntjlbptYRE+6YIpba1V4hslKrhAQfqkACiEg8paokn+3byHPWv
EVopvwKBgEoXjcvplHwUp7wbSodJjbd03wBSYM5aCSROViX7DYrpQxwQ9mWYrsUN
3n3DB4vXYC7XIU9BkffsQ+URRdE9YjtyI3F23wJ+G0Oon2Tgg/AMWtAEFEDbKAAF
zYdBcg1ivE25jwrK/R8BAkGxcJbzORvQ+3fwoxt6Vf0lTcKlXbet      
-----END RSA PRIVATE KEY-----
"#;