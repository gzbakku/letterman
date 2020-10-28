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
client api is used to send emails, base client lib is not async compatible but its pretty fast if you want to use this with tokio see the nsync lib below.
```rust
use letterman::client;

fn main() {

    println!(">>> sending mail");

    //--------------------------------------
    //start the email builder

    let mut email = client::Email::new();

    //--------------------------------------
    //all of the following methods are necessary to call although lib does not check if they are provided or not.

    email.name(String::from("gzbakku"));
    email.from(String::from("gzbakku@gmail.com"));
    email.to(String::from("akku@localhost"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\nsecond message\nthird message"));

    //--------------------------------------
    //define the body here just call the is_html method on email if you provide valid html in the body method

    // email.body(String::from("<html> <header><title>This is title</title></header> <body> Hello world </body> </html>"));
    // email.is_html();

    //--------------------------------------
    //you can add files as base64 encoded, only use base64 value without base64 header

    // let base64_data = "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAAdgAAAHYBTnsmCAAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAAJfSURBVDiNpZLdS1MBGMafc87OPtx23M7Opse2hDb8mB+ZLRC70IhBd4GSJAUVYnXVXyBBEHST1xZRmUgXgdHFiC6KwixCyjLDmTKx4cm17Ux2trntnO3YTYrOFYTP3fvwvj8eeF5gn6L+4pPtza6bvJ3xrcWkd/9NrXXaep49vCKP3Dqbomn68Cl/w52BC8emJsYvdpTuasoB0tl88Gngc0TK5GRFUSrP9fou9fZ5tQ8eTY8COAIg+0+AKKaDYxPTbQAUAAeK+U0SKoGVlXXDH29bxM6Bc/B3K4zWWlUtZIuKnC0UCxsESSotddZBzyEXNbcQTi2GxMfxqHC1XAJSpzd1eVs76g0VZtC0dleqaE4G43CYdcLbbgAkALUUoArhpf4ui+Wj286TyXwOObUIANCTFCp1OoRiEVUIL/VvHW8DWI4fZBh2QJLEF7yd/9njO+7UECTYCiMAILGRQWFTxfinKYHlqk8zjG1EkhL3E/G1exqLveqat7njRo3LY1lbDfli+bQa2chg6ZeA+UQcAIEmloOnqgbRXL66qbVziHe6qdXwYv383Ac9yZisgzUujwUATJUspXewtH94CG5vI2aTEmaTSbibGuEfHoLBztJmhqUAwHmwzsIw7GXN+np89OXzsQaG4VoUOZdWOFtbKjBpO2ricLvLDwBwG21IBSaxHFoWgwnxC63VmyQpPqcUisFdNQLAyfrm133tnd3l/uPJzPs3r75/O7HT2/NIZkpbrCUIGEtqTMt5GAhSLd3fA5gRQmd+1LVcD0uCOyRGrVt+RpGlwMLX8+WS7Uu/AV/Q4yOF5rS7AAAAAElFTkSuQmCC".to_string();
    // email.attach_base64("drink.png".to_string(),base64_data);

    //--------------------------------------
    //attach a file that is locally available on the server

    // email.attach("d://workstation/expo/rust/letterman/run.bat".to_string());

    //--------------------------------------
    //use cc bcc values as provided below

    // email.cc("gzbtejasav@gmail.com".to_string());
    // email.cc("nodemailer@gmail.com".to_string());

    // email.bcc("gzbtejasav@gmail.com".to_string());
    // email.bcc("nodemailer@gmail.com".to_string());

    match email.send(){
        Ok(_)=>{
            println!("email sent");
        },
        Err(_)=>{
            println!("email failed");
        }
    }

}

```

### nsync lib is compatible with tokio version 0.2.22

```rust
use letterman::client::nsync::Email;

#[tokio::main]
fn start_async() {

    println!(">>> sending mail");

    let mut email = Email::new();
    email.name(String::from("gzbakku"));
    email.from(String::from("gzbakku@gmail.com"));
    email.to(String::from("akku@localhost"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\nsecond message\nthird message"));

    //--------------------------------------
    //just await on the send method on the email

    match email.send().await{
        Ok(_)=>{
            println!("email sent");
        },
        Err(_)=>{
            println!("email failed");
        }
    }

}

```
