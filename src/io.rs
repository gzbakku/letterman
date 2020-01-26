use std::io::prelude::*;
use std::net::{TcpStream};
use std::fs::File;
use crate::common::{error};
use base64;

#[derive(Debug)]
pub struct READ {
    pub result:bool,
    pub code:u16,
    pub message:String
}

pub fn send_file(path:&str,stream:&mut TcpStream) -> Result<(),String> {

    match File::open(path) {
        Ok(mut reader)=>{
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer) {
                Ok(_)=>{
                    match send_only_bytes(stream,buffer) {
                        Ok(_)=>{
                            return Ok(());
                        },
                        Err(_)=>{
                            return Err(error("failed-send_only_bytes-send_file"));
                        }
                    }
                },
                Err(e)=>{
                    println!("error : {:?}",e);
                    return Err(error("failed-read_file-send_file"));
                }
            }
        },
        Err(e)=>{
            println!("error : {:?}",e);
            return Err(error("failed-open_file-send_file"));
        }
    }

}

pub fn send_only_bytes(stream:&mut TcpStream,data:Vec<u8>) -> Result<(),String> {
    let encoded = base64::encode(&data);
    println!("encoded : {:?}",encoded);
    match stream.write_all(&encoded.as_bytes()){
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err(error("failed-send-send_result"));
        }
    }
}

pub fn send_only(stream:&mut TcpStream,m:String) -> Result<(),String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str(" \r\n");
    }
    match stream.write_all(&c.as_bytes()){
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err(error("failed-send-send_result"));
        }
    }
}

pub fn send(stream:&mut TcpStream,m:String) -> Result<READ,String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str(" \r\n");
    }
    match stream.write_all(&c.as_bytes()){
        Ok(_)=>{
            match read(stream) {
                Ok(m)=>{
                    return Ok(m);
                },
                Err(_)=>{
                    return Err(error("failed-read-send_result"));
                }
            }
        },
        Err(_)=>{
            return Err(error("failed-send-send_result"));
        }
    }
}

pub fn read(stream:&mut TcpStream) -> Result<READ,String> {

    let mut line = String::new();
    let mut buff = [0; 1000];
    while match stream.read(&mut buff) {
        Ok(_)=>{
            match String::from_utf8(buff.to_vec()) {
                Ok(result)=>{
                    if result.contains("\r\n") {
                        line.push_str(&result);
                        false
                    } else {
                        line.push_str(&result);
                        true
                    }
                },
                Err(_)=>{
                    return Err(error("failed-parse_byte_array-to_string-read"));
                }
            }
        },
        Err(_)=>{
            return Err(error("failed read 10 bytes"));
        }
    } {}



    let line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();

    let mut letter = line_vec[0].to_string();
    let mut parsed = READ {
        result:true,
        code:100,
        message:String::new()
    };

    let code_as_string = &mut letter[0..3].to_string();
    match code_as_string.parse::<u16>() {
        Ok(r)=>{
            parsed.code = r;
        },
        Err(_)=>{
            println!("response : {:?}",letter);
            return Err(error("failed-parse_code"));
        }
    }

    if parsed.code > 354 || parsed.code < 200 {
        parsed.result = false;
    }

    let message_as_string = &mut letter[4..].to_string();
    parsed.message = message_as_string.clone();
    if false {
        println!("parsed : {:?}",parsed);
    }

    return Ok(parsed);

}
