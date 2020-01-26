use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod common;
mod io;
use common::{error};

fn main() {
    println!("Hello, world!");
    start();
}

#[derive(Debug,Clone)]
struct ACTION {
    tag:&'static str,
    cate:&'static str,
    io:&'static str,
    cmd:&'static str
}

fn start(){

    let addr = "127.0.0.1:25";

    let mut stream;
    match TcpStream::connect(addr) {
        Ok(r)=>{
            stream = r;
        },
        Err(e)=>{
            println!("error : {:?}",e);
            error("failed to connect to fake smtp");
            return;
        }
    }

    let mut actions = vec!(ACTION { io:"read", cate:"cmd", tag:"connect", cmd:"" });

    if false {
        actions.push(ACTION { io:"write", cate:"cmd", tag:"start tls", cmd:"STARTTLS" });
    }

    if true {
        actions.push(ACTION { io:"write", cate:"cmd", tag:"say hellow", cmd:"EHLO localhost" });
    }

    if true {
        actions.push(ACTION { io:"write", cate:"cmd", tag:"from", cmd:"MAIL FROM:<akku@localhost>" });
        actions.push(ACTION { io:"write", cate:"cmd", tag:"to", cmd:"RCPT TO:<tejasav@localhost>" });
    }

    execute(actions.to_vec(),&mut stream);

    actions = Vec::new();

    if true {
        actions.push(ACTION { io:"write", cate:"cmd", tag:"data", cmd:"DATA" });

        actions.push(ACTION { io:"write", cate:"data", tag:"data-date", cmd:"Date: Thu, 21 May 2008 05:33:29 -0700" });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-from", cmd:"From: SamLogic <mail@samlogic.com>" });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-subject", cmd:"Subject: The Next Meeting" });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-to", cmd:"To: john@mail.com" });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-empty", cmd:"" });

        actions.push(ACTION { io:"write", cate:"data", tag:"data-body", cmd:"Hi John" });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-body", cmd:"The next meeting will be on Friday." });
        actions.push(ACTION { io:"write", cate:"data", tag:"data-body", cmd:"/Anna." });

        execute(actions.to_vec(),&mut stream);
        actions = Vec::new();

        if true {

            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-mime", cmd:"MIME-Version: 1.0" });
            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-content_type", cmd:"Content-Type:multipart/mixed;boundary=\"KkK170891tpbkKk__FV_KKKkkkjjwq\"" });

            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-border-start", cmd:"--KkK170891tpbkKk__FV_KKKkkkjjwq" });

            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-content-details", cmd:"Content-Type:application/octet-stream;name=\"make.txt\"" });
            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-content-encoding", cmd:"Content-Transfer-Encoding:base64" });
            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-content-disposition", cmd:"Content-Disposition:attachment;filename=\"make.txt\"" });

            execute(actions.to_vec(),&mut stream);
            actions = Vec::new();

            let file_path = "C:/Users/tejas/Desktop/make.txt";

            match io::send_file(file_path,&mut stream) {
                Ok(_)=>{
                    println!(">>> file sent");
                },
                Err(_)=>{
                    error("failed-send_file");
                    return;
                }
            }

            actions.push(ACTION { io:"write", cate:"data", tag:"data-file-border-end", cmd:"/--KkK170891tpbkKk__FV_KKKkkkjjwq--" });

        }

        actions.push(ACTION { io:"write", cate:"cmd", tag:"data-finish", cmd:"\r\n.\r\n" });

        execute(actions.to_vec(),&mut stream);
        actions = Vec::new();

        //actions.push(ACTION { io:"write", cate:"cmd", tag:"data-date", cmd:"" });



    }



}

fn execute(actions:Vec<ACTION>,mut stream:&mut TcpStream){

    for action in actions {
        if action.io == "read" {
            match io::read(&mut stream){
                Ok(r)=>{
                    println!("read action : {:?}, result : {:?}",action.tag,r);
                    if r.result == false {
                        println!("action : {:?}",action);
                        error("failed-execute");
                        break;
                    }
                },
                Err(_)=>{
                    println!("failed-read-execute-for => {:#?}",action);
                    break;
                }
            }
        } else {
            if action.cate == "data" {
                match io::send_only(&mut stream,action.cmd.to_string()){
                    Ok(_)=>{
                        println!("write data : {:?}",action.tag);
                    },
                    Err(_)=>{
                        println!("failed-write-execute-for => {:#?}",action);
                        break;
                    }
                }
            } else {
                match io::send(&mut stream,action.cmd.to_string()){
                    Ok(r)=>{
                        println!("write action : {:?}, result : {:?}",action.tag,r);
                        if r.result == false {
                            println!("action : {:?}",action);
                            error("failed-execute");
                            break;
                        }
                    },
                    Err(_)=>{
                        println!("failed-write-execute-for => {:#?}",action);
                        break;
                    }
                }
            }
        }
    }

}
