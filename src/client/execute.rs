use crate::io;
use crate::client::{Action,Email,parse};
use std::net::TcpStream;

pub fn send_mail(email:Email) -> Result<(),&'static str> {

    let actions:Vec<Action>;
    match parse::init(email){
        Ok(a)=>{
            actions = a;
        },
        Err(_)=>{
            return Err("failed-parse_mail");
        }
    }

    let addr = "127.0.0.1:25";
    let mut stream;
    match TcpStream::connect(addr) {
        Ok(r)=>{
            stream = r;
        },
        Err(e)=>{
            println!("error : {:?}",e);
            return Err("failed-start-connection");
        }
    }

    for action in actions {
        if action.io == "read" {
            match io::read(&mut stream){
                Ok(r)=>{
                    println!("read action : {:?}, result : {:?}",action.tag,r);
                    if r.result == false {
                        println!("action : {:?}",action);
                        return Err("failed-execute");
                    }
                },
                Err(_)=>{
                    println!("failed-read-execute-for => {:#?}",action);
                    return Err("failed-read");
                }
            }
        } else {//write actions here
            if action.cate == "data" {
                match io::send_only(&mut stream,action.cmd.to_string()){
                    Ok(_)=>{
                        // println!("write data : {:?}",action.tag);
                    },
                    Err(_)=>{
                        println!("failed-write-execute-for => {:#?}",action);
                        return Err("failed-write-data");
                    }
                }
            } else {
                match io::send(&mut stream,action.cmd.to_string()){
                    Ok(r)=>{
                        println!("write action : {:?}, result : {:?}",action.tag,r);
                        if r.result == false {
                            println!("action : {:?}",action);
                            return Err("failed-write-cmd-result");
                        }
                    },
                    Err(_)=>{
                        println!("failed-write-execute-for => {:#?}",action);
                        return Err("failed-write-cmd");
                    }
                }
            }
        }
    }//loop

    return Ok(());

}
