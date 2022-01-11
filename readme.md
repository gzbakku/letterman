# letterman - rust

this is a grounds up smtp client and server lib for rust, trust-dns-resolver is used to resolve the mx records of the domains if no records are found smtp is tried on the given domain if not it fails. native-tls is used to support start tls smtp commands works with pretty much all email providers.

## domain setup

set spf, dkim and dmarc reocrds on your domain for verifiable email delivery and reception.

## Supported SMTP features/extensions

- RDNS Validation
- DKIM Validation
- SPF Lookup
- 8BITMIME
- Alternative Body
- Attachments
- Base64/Quoted-printable Decoding

### testing smtp server

you can test dkim validation, spf lookup and email body validation on dkimvalidator.com

***please support the creators of testing servers***

# Client Api
client api supports pipelinening and can send multiple smtp commands in batch for faster resolution, dkim is required for mail delivery.

```rust
use std::time::Instant;
use letterman::client::{read_key,Connection,Email};

#[tokio::main]
async fn main() {

    println!(">>> sending mail async");

    //this is private dkim key for which public key is published at a dkim subdomain as a txt record.
    let key:String;
    match read_key("../secret/private.key".to_string()).await{
        Ok(v)=>{
            key = v;
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return;
        }
    }

    let mut conn:Connection;
    match Connection::new(
        String::from("localhost"),                      //receiver domain
        String::from("mailcenter.herokuapp.com"),       //server name
        key,                                            //dkim private key
        String::from("dkim"),                           //dkim txt value name
        String::from("silvergram.in"),                  //sender domain
    ){
        Ok(v)=>{conn = v;},
        Err(_)=>{
            return;
        }
    }

    //add emails to this connection
    //emails are parsed before the connection si even started
    //large number of emails in one connection will delay for parsing all emails
    for i in 0..1{
        conn.add(build_mail_new(i.to_string()));
    }

    let hold = Instant::now();

    if true{
        match conn.send().await{
            Ok(_v)=>{
                println!("send successfull  : {:?} {:?}",_v.0,_v.1);
            },
            Err(_e)=>{
                println!("send failed : {:?}",_e);
            }
        }
    }

    println!("finished in  : {:?}",hold.elapsed());

}

fn build_mail_new(tracking_id:String) -> Email{

    let mut email = Email::new();

    email.server_name(String::from("mailcenter.herokuapp.com"));
    email.name(String::from("gzbakku"));
    email.from(String::from("akku@silvergram.in"));
    email.tracking_id(tracking_id);
    email.to(String::from("gzbakku@localhost"));
    //the receivers will receive the message this feature allows cc and bcc smtp functions
    email.receiver(String::from("gzbakku@localhost"));
    email.receiver(String::from("gzbakku1@localhost"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\r\nsecond message\r\nthird message"));

    //add html body
    if 1 == 1 {
        email.html(
            String::from(
                "<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"
            )
        );
    }

    //attach a file
    if 1 == 1 {
        email.attach("d://workstation/expo/rust/letterman/letterman/drink.png".to_string());
    }

    //attach a base64 encoded file
    if 1 == 1 {
        let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
        email.attach_base64("drink_1.png".to_string(),base64_data,"image/png".to_string());
    }

    return email;

}

```

# Server Api
server api supports rdns,spf and dkim validation with pipelining extension.
```rust

use letterman::server::config::{ServerConfig};
use letterman::server::{init,CheckMail,ProcessMail};
use letterman::json::JsonValue;
use letterman::flume::unbounded as FlumeChannel;
use letterman::flume::Sender as FlumeSender;

#[tokio::main]
async fn main(){

    let config:ServerConfig;
    match ServerConfig::new(
        vec![587,2525],
        String::from("silversender.com"),
        format!("../secret/end.cert"),
        format!("../secret/end.rsa"),
        100_000,
        String::from("../letter_man_que/que/que.akku"),
        5_000_000,
        5,
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

    match init(
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

    Ok(true)

}

```
