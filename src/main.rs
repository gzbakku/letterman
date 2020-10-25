mod common;
mod io;
mod client;

fn main() {

    println!(">>> sending mail");

    let mut email = client::Email::new();
    email.name(String::from("gzbakku"));
    email.from(String::from("akku@silvergram.in"));
    email.to(String::from("gzbakku@gmail.com"));
    email.subject(String::from("hello world"));
    email.body(String::from("first message\nsecond message\nthird message"));
    // email.body(String::from("<html> <header><title>This is title</title></header> <body> Hello world </body> </html>"));
    // email.attach("d://workstation/expo/rust/letterman/run.bat".to_string());
    // email.attach("d://workstation/expo/rust/letterman/sample.txt".to_string());
    // email.attach("d://workstation/expo/rust/letterman/MailHog_windows_386.exe".to_string());
    // email.is_html();

    match email.send(){
        Ok(_)=>{
            println!("email sent");
        },
        Err(e)=>{
            println!("email failed : {:?}",e);
        }
    }

}
