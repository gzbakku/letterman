use std::io::prelude::*;
use std::net::{TcpStream};
use std::fs::File;
use crate::common::{error};
use base64;
use rustls::{ClientSession};

pub fn get_file_name(path:String) -> Result<String,&'static str>{

    let mut pool:Vec<&str>;
    if path.contains("\\"){
        pool = path.split("\\").collect::<Vec<&str>>();
    } else if path.contains("/"){
        pool = path.split("/").collect::<Vec<&str>>();
    } else {
        return Ok(path);
    }

    match pool.pop(){
        Some(v)=>{
            return Ok(v.to_string());
        },
        None=>{
            return Err("failed-extract-file_name");
        }
    }

}

pub struct Reader{
    pub data:String,
    pub name:String
}

pub fn read_file(path:String) -> Result<Reader,&'static str>{
    match File::open(path.clone()) {
        Ok(mut reader)=>{
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer) {
                Ok(_)=>{
                    match get_file_name(path){
                        Ok(file_name)=>{
                            return Ok(Reader {
                                data:base64::encode(&buffer),
                                name:file_name
                            });
                        },
                        Err(_)=>{
                            return Err("failed-get_file_name");
                        }
                    }
                },
                Err(_)=>{
                    return Err("failed-read_file");
                }
            }
        },
        Err(_)=>{
            return Err("failed-open_file");
        }
    }
}

pub fn send_only(stream:&mut TcpStream,m:String) -> Result<(),String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
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
        c.push_str("\r\n");
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

pub fn read(stream:&mut TcpStream) -> Result<READ,&'static str> {


    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match stream.read(&mut buff) {
            Ok(len)=>{
                println!("len : {:?}",len);
                for i in 0..len{collect.push(buff[i].clone());}
                if len < 5000 {break;}
                // if len == 0{break;}
            },
            Err(_)=>{
                return Err("failed read 10 bytes");
            }
        }
    }

    let line:String;
    match String::from_utf8(collect) {
        Ok(result)=>{
            line = result;
        },
        Err(_)=>{
            return Err("failed-parse_byte_array-to_string-read");
        }
    }

    let mut line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();

    let mut features = Vec::new();
    if line_vec.len() > 2 {
        let mut features_index = 0;
        for feature in line_vec.iter(){
            if features_index > 0{
                if feature.len() > 0{
                    match parse_feature(&mut feature.to_string()){
                        Ok(f)=>{features.push(f);},
                        Err(_)=>{}
                    }
                }
            }
            features_index = features_index + 1;
        }
    }

    match parse(&mut line_vec[0].to_string(),features){
        Ok(p)=>{
            return Ok(p);
        },
        Err(_)=>{
            return Err("failed-parse_response");
        }
    }

}

pub fn secure_send_only(stream:&mut ClientSession,m:String) -> Result<(),String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
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

pub fn secure_send(stream:&mut ClientSession,m:String) -> Result<READ,String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
    }
    match stream.write_all(&c.as_bytes()){
        Ok(_)=>{
            match secure_read(stream) {
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

pub fn secure_read(stream:&mut ClientSession) -> Result<READ,&'static str> {


    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match stream.read(&mut buff) {
            Ok(len)=>{
                for i in 0..len{collect.push(buff[i].clone());}
                if len < 5000 {break;}
            },
            Err(_)=>{
                return Err("failed read 10 bytes");
            }
        }
    }

    let line:String;
    match String::from_utf8(collect) {
        Ok(result)=>{
            line = result;
        },
        Err(_)=>{
            return Err("failed-parse_byte_array-to_string-read");
        }
    }

    // println!("\n{:?}\n",line);

    let mut line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();

    let mut features = Vec::new();
    if line_vec.len() > 2 {
        let mut features_index = 0;
        for feature in line_vec.iter(){
            if features_index > 0{
                if feature.len() > 0{
                    match parse_feature(&mut feature.to_string()){
                        Ok(f)=>{features.push(f);},
                        Err(_)=>{}
                    }
                }
            }
            features_index = features_index + 1;
        }
    }

    match parse(&mut line_vec[0].to_string(),features){
        Ok(p)=>{
            return Ok(p);
        },
        Err(_)=>{
            return Err("failed-parse_response");
        }
    }

}

#[derive(Debug)]
pub struct READ {
    pub result:bool,
    pub code:u16,
    pub message:String,
    pub features:Vec<Feature>
}

#[derive(Debug)]
pub struct Feature {
    pub result:bool,
    pub code:u16,
    pub message:String,
    pub _type:String,
    pub _val:String
}

fn parse_feature(letter:&mut String) -> Result<Feature,&'static str>{
    let mut parsed = Feature {
        result:true,
        code:100,
        message:String::new(),
        _type:String::new(),
        _val:String::new()
    };
    let code_as_string = &mut letter[0..3].to_string();
    match code_as_string.parse::<u16>() {
        Ok(r)=>{
            parsed.code = r;
        },
        Err(_)=>{
            return Err("failed-parse_code");
        }
    }
    if parsed.code > 354 || parsed.code < 200 {
        parsed.result = false;
    }
    let message_as_string = &mut letter[4..].to_string();
    parsed.message = message_as_string.clone();
    if message_as_string.contains(" "){
        let val_split: Vec<&str> = message_as_string.split(" ").collect::<Vec<&str>>();
        parsed._type = val_split[0].to_string();
        parsed._val = val_split[1].to_string();
    } else {
        parsed._type = message_as_string.to_string();
    }
    return Ok(parsed);
}

fn parse(letter:&mut String,features:Vec<Feature>) -> Result<READ,&'static str>{
    println!(">>>>>>>>>>>>>>> {:?}",letter);
    if letter.len() <= 4 {
        return Err("empty message");
    }
    let mut parsed = READ {
        result:true,
        code:100,
        message:String::new(),
        features:features
    };
    let code_as_string = &mut letter[0..3].to_string();
    match code_as_string.parse::<u16>() {
        Ok(r)=>{
            parsed.code = r;
        },
        Err(_)=>{
            return Err("failed-parse_code");
        }
    }
    if parsed.code > 354 || parsed.code < 200 {
        parsed.result = false;
    }
    let message_as_string = &mut letter[4..].to_string();
    parsed.message = message_as_string.clone();
    return Ok(parsed);
}
