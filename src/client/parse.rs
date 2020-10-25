use crate::client::{Action,Email};
use crate::client::build::BaseFile;
use crate::io::read_file;

pub fn init(email:Email) -> Result<Vec<Action>,&'static str>{

    let mut actions = vec!();

    if !email.from.contains("@") {return Err("invalid_from_email");}
    let mut domain:String = email.from.split("@").collect::<Vec<&str>>()[1].to_string();
    if domain.contains("localhost"){domain = String::from("localhost");} else{
        if !domain.contains("."){return Err("invalid-from-domain");}
    }

    //connection
    actions.push(Action { io:"read", cate:"cmd", tag:"connect", cmd:"".to_string() });
    actions.push(Action { io:"write", cate:"cmd", tag:"say_hello", cmd:format!("EHLO {}",domain) });

    // return Ok(actions);

    actions.push(Action { io:"write", cate:"cmd", tag:"from", cmd:format!("MAIL FROM:<{}>",email.from) });
    actions.push(Action { io:"write", cate:"cmd", tag:"to", cmd:format!("RCPT TO:<{}>",email.to) });

    if email.attach.len() > 0 || email.attach_base64.len() > 0{
        for i in get_dynamic_header(email.name,email.from,email.to,email.subject,email.cc){
            actions.push(i);
        }
        if !email.is_html{
            for i in get_text_body(email.body.clone(),true){
                actions.push(i);
            }
        } else {
            for i in get_html_body(email.body.clone(),true){
                actions.push(i);
            }
        }
        actions.push(Action { io:"write", cate:"data", tag:"dh-empty", cmd:format!("") });
        for i in email.attach.iter(){
            match get_file_body(i.to_string()){
                Ok(pool)=>{
                    for j in pool.iter(){
                        actions.push(j.clone());
                    }
                },
                Err(_)=>{
                    return Err("failed-get_file_body");
                }
            }
        }
        for i in email.attach_base64.iter(){
            for j in get_base_64_body(&i).iter(){
                actions.push(j.clone());
            }
        }
        actions.push(Action { io:"write", cate:"data", tag:"end_body", cmd:format!("--e6279a8adea1bd6ce96812378072940a--") });
    } else {

        for i in get_simple_header(email.name,email.from,email.to,email.subject,email.cc){
            actions.push(i);
        }
        if !email.is_html{
            for i in get_text_body(email.body.clone(),false){
                actions.push(i);
            }
        } else {
            for i in get_html_body(email.body.clone(),false){
                actions.push(i);
            }
        }
        actions.push(Action { io:"write", cate:"data", tag:"dh-empty", cmd:format!("") });
    }

    actions.push(Action { io:"write", cate:"cmd", tag:"data-finish", cmd:"\r\n.\r\n".to_string() });
    actions.push(Action { io:"write", cate:"cmd", tag:"quit", cmd:"QUIT".to_string() });

    return Ok(actions);

}

fn get_base_64_body(base:&BaseFile) -> Vec<Action>{
    let mut actions = vec!();
    actions.push(Action { io:"write", cate:"data", tag:"fb-starter", cmd:format!("--e6279a8adea1bd6ce96812378072940a") });
    actions.push(Action { io:"write", cate:"data", tag:"fb-content_type", cmd:format!("Content-Type: application/octet-stream; name=\"{}\"",base.name.to_string()) });
    actions.push(Action { io:"write", cate:"data", tag:"fb-encoding", cmd:format!("Content-Transfer-Encoding: base64") });
    actions.push(Action { io:"write", cate:"data", tag:"fb-declare", cmd:format!("Content-Disposition: attachment") });
    actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    actions.push(Action { io:"write", cate:"data", tag:"fb-data", cmd:base.base64.to_string() });
    return actions;
}

fn get_file_body(path:String) -> Result<Vec<Action>,&'static str>{
    match read_file(path){
        Ok(file)=>{
            let mut actions = vec!();
            actions.push(Action { io:"write", cate:"data", tag:"fb-starter", cmd:format!("--e6279a8adea1bd6ce96812378072940a") });
            actions.push(Action { io:"write", cate:"data", tag:"fb-content_type", cmd:format!("Content-Type: application/octet-stream; name=\"{}\"",file.name) });
            actions.push(Action { io:"write", cate:"data", tag:"fb-encoding", cmd:format!("Content-Transfer-Encoding: base64") });
            actions.push(Action { io:"write", cate:"data", tag:"fb-declare", cmd:format!("Content-Disposition: attachment") });
            actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
            actions.push(Action { io:"write", cate:"data", tag:"fb-data", cmd:format!("{}",file.data) });

            return Ok(actions);
        },
        Err(_)=>{
            return Err("failed-read_file");
        }
    }
}

fn get_html_body(message:String,multipart:bool) -> Vec<Action>{
    let mut actions = vec!();
    if multipart{
        actions.push(Action { io:"write", cate:"data", tag:"tb-starter", cmd:format!("--e6279a8adea1bd6ce96812378072940a") });
    }
    actions.push(Action { io:"write", cate:"data", tag:"tb-content_type", cmd:"Content-Type: text/html; charset=UTF-8".to_string() });
    actions.push(Action { io:"write", cate:"data", tag:"connect", cmd:format!("Content-Transfer-Encoding: 8bit") });
    actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    actions.push(Action { io:"write", cate:"data", tag:"tb-message", cmd:format!("{}",message) });
    return actions;
}

fn get_text_body(message:String,multipart:bool) -> Vec<Action>{
    let mut actions = vec!();
    if multipart{
        actions.push(Action { io:"write", cate:"data", tag:"tb-starter", cmd:format!("--e6279a8adea1bd6ce96812378072940a") });
        actions.push(Action { io:"write", cate:"data", tag:"tb-content_type", cmd:"Content-Type: text/plain; charset=\"iso-8859-1\"".to_string() });
        actions.push(Action { io:"write", cate:"data", tag:"tb-encoding_type", cmd:format!("Content-Transfer-Encoding: 8bit") });
        actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    } else {
        actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    }
    actions.push(Action { io:"write", cate:"data", tag:"tb-message", cmd:format!("{}",message) });
    return actions;
}

fn get_dynamic_header(name:String,from:String,to:String,subject:String,cc:String) -> Vec<Action>{
    let mut actions = vec!();
    actions.push(Action { io:"write", cate:"cmd", tag:"data", cmd:"DATA".to_string() });
    actions.push(Action { io:"write", cate:"data", tag:"dh-date", cmd:format!("Date: Fri, 02 Oct 2020 14:19:41 +0200") });
    actions.push(Action { io:"write", cate:"data", tag:"dh-subject", cmd:format!("Subject: {}",subject) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-to", cmd:format!("To: {}",to) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-from", cmd:format!("From: {} <{}>",name,from) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-from", cmd:format!("Cc: {}",cc) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-mime", cmd:"MIME-Version: 1.0".to_string() });
    actions.push(Action { io:"write", cate:"data", tag:"dh-content_type", cmd:format!("Content-Type: multipart/mixed; boundary=\"{}\"","e6279a8adea1bd6ce96812378072940a") });
    actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    return actions;
}

fn get_simple_header(name:String,from:String,to:String,subject:String,cc:String) -> Vec<Action>{
    let mut actions = vec!();
    actions.push(Action { io:"write", cate:"cmd", tag:"data", cmd:"DATA".to_string() });
    actions.push(Action { io:"write", cate:"data", tag:"dh-date", cmd:format!("Date: Fri, 02 Oct 2020 14:19:41 +0200") });
    actions.push(Action { io:"write", cate:"data", tag:"dh-subject", cmd:format!("Subject: {}",subject) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-to", cmd:format!("To: {}",to) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-from", cmd:format!("From: {} <{}>",name,from) });
    actions.push(Action { io:"write", cate:"data", tag:"dh-from", cmd:format!("Cc: {}",cc) });
    // actions.push(Action { io:"write", cate:"data", tag:"dh-spacer", cmd:format!("") });
    return actions;
}
