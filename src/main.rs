mod common;
mod io;
mod client;

fn main() {

    println!(">>> sending mail");

    let mut email = client::Email::new();
    email.name(String::from("gzbakku"));
    email.from(String::from("gzbakku@gmail.com"));
    email.to(String::from("akku@localhost"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\nsecond message\nthird message"));
    // email.body(String::from("<html> <header><title>This is title</title></header> <body> Hello world </body> </html>"));
    // email.attach("d://workstation/expo/rust/letterman/run.bat".to_string());
    // email.attach("d://workstation/expo/rust/letterman/sample.txt".to_string());
    // email.attach("d://workstation/expo/rust/letterman/MailHog_windows_386.exe".to_string());
    // email.is_html();

    email.cc("gzbtejasav@gmail.com".to_string());
    email.cc("nodemailer@gmail.com".to_string());

    email.bcc("gzbtejasav@gmail.com".to_string());
    email.bcc("nodemailer@gmail.com".to_string());

    match email.send(){
        Ok(_)=>{
            println!("email sent");
        },
        Err(_)=>{
            println!("email failed");
        }
    }

}
