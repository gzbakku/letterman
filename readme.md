# letterman - rust

this is a grounds up smtp client and server lib for rust, trust-dns-resolver is used to resolve the mx records of the domains if no records are found smtp is tried on the given domain if not it fails. native-tls is used to support start tls smtp commands works with pretty much all email providers.

## domain setup

set spf, dkim and dmarc reocrds on your domain for better email delivery

## upcoming features

  -full support for spf,dkim and dmarc management
  -smtp server for incoming emails

### testing smtp server

download the test smtp server given below to test client lib features

https://github.com/rnwood/smtp4dev

# Client Api
client api is used to send emails, base client lib supports tokio with send_tokio function

```rust
use letterman::client;

fn main(){
    start_sync();
    if false {
        main_async();
    }
}

#[tokio::main]
async fn main_async(){
    start_async().await;
}

async fn start_async() {
    println!(">>> sending mail async");
    let email = build_mail();
    match email.send_tokio().await{
        Ok(_)=>{
            println!("email sent from tokio");
        },
        Err(e)=>{
            println!("email failed : {:?}",e);
        }
    }
}

fn start_sync() {
    println!(">>> sending mail sync");
    let email = build_mail();
    match email.send(){
        Ok(_)=>{
            println!("email sent");
        },
        Err(e)=>{
            println!("email failed : {:?}",e);
        }
    }
}

fn build_mail() -> client::Email{

    let mut email = client::Email::new();

    match client::read_key("d://workstation/expo/rust/letterman/keys/private.key".to_string()){
        Ok(key)=>{
            email.private_key(key);
        },
        Err(e)=>{
            println!("!!! {:?}",e);
        }
    }

    email.dkim_selector(String::from("dkim"));
    email.server_name(String::from("mailcenter.herokuapp.com"));
    email.name(String::from("gzbakku"));
    email.from(String::from("akku@silvergram.in"));
    email.to(String::from("akku@localhost:25"));
    // email.to(String::from("gzbakku@gmail.com"));
    email.to(String::from("9mvxra2pyfknnz@dkimvalidator.com"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\r\nsecond message\r\nthird message"));

    if 1 == 1 {
        email.body(String::from("<html> <header><title>This is title</title></header> <body> <h1>Hello world</h1> </body> </html>"));
        email.is_html();
    }

    if 1 == 0 {
        email.attach("d://workstation/expo/rust/letterman/drink.png".to_string());
    }

    if false {
        let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
        email.attach_base64("drink.png".to_string(),base64_data);
    }

    return email;

}

```
