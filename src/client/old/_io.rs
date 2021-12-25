use std::io::prelude::*;
use std::net::{TcpStream};
use std::fs::File;
use crate::common::{error};
use base64;
use native_tls::TlsStream;
use tokio_rustls::server::TlsStream as TokioRustLSTlsStream;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use regex::Regex;
use rustls_pemfile::{certs,rsa_private_keys};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use std::io::{BufReader};
// use tokio::time::timeout;
// use std::time::Duration;

const PRINT_MESSAGES:bool = true;
const PRINT_DATA_MESSAGES:bool = false;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct Reader{
    pub data:String,
    pub name:String
}

#[allow(dead_code)]
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

pub fn read_as_text(path:String) -> Result<String,&'static str>{
    match read_file_raw(path){
        Ok(buffer)=>{
            match String::from_utf8(buffer){
                Ok(d)=>{return Ok(d);},
                Err(_)=>{return Err("failed-parse_buffer_to_text")}
            }
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return Err("failed-read_raw_file");
        }
    }
}

pub fn read_file_raw(path:String) -> Result<Vec<u8>,&'static str>{
    match File::open(path) {
        Ok(mut reader)=>{
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer) {
                Ok(_)=>{
                    return Ok(buffer);
                },
                Err(_)=>{
                    return Err("failed-read_file");
                }
            }
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return Err("failed-open_file");
        }
    }
}

#[allow(dead_code)]
pub fn send_only(stream:&mut TcpStream,m:String) -> Result<(),String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
    } else {
        let hold = c.split("\r\n").collect::<Vec<&str>>();
        if hold[hold.len() - 1].len() > 0{
            c.push_str("\r\n");
        }
    }
    if PRINT_MESSAGES && PRINT_DATA_MESSAGES{
        println!("{:?}",c);
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

#[allow(dead_code)]
pub fn send(stream:&mut TcpStream,m:String) -> Result<READ,String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
    }
    if PRINT_MESSAGES{
        println!("{:?}",c);
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

#[allow(dead_code)]
pub fn read(stream:&mut TcpStream) -> Result<READ,&'static str> {

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

    if PRINT_MESSAGES{
        println!("{:?}",line);
    }

    let line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();

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

#[allow(dead_code)]
pub async fn read_server(stream:&mut TokioRustLSTlsStream<TokioTcpStream>,r:&Regex) -> Result<Vec<String>,&'static str> {

    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match stream.read(&mut buff).await {
            Ok(len)=>{
                for i in 0..len{collect.push(buff[i].clone());}
                if len < 5000 {break;}
            },
            Err(e)=>{
                println!("failed read bytes : {:?}",e);
                return Err("failed read bytes");
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

    if PRINT_MESSAGES{
        println!("{:?}",line);
    }

    match parse_request(&line, &r){
        Ok(v)=>{
            return Ok(v);
        },
        Err(_)=>{
            return Err("failed-parse_byte_array-to_string-read");
        }
    }

}

#[allow(dead_code)]
pub async fn read_only_server(stream:&mut TokioRustLSTlsStream<TokioTcpStream>) -> Result<String,&'static str> {

    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match stream.read(&mut buff).await {
            Ok(len)=>{
                for i in 0..len{collect.push(buff[i].clone());}
                if len < 5000 {break;}
            },
            Err(e)=>{
                println!("failed read bytes : {:?}",e);
                return Err("failed read bytes");
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

    if PRINT_MESSAGES{
        println!("{:?}",line);
    }

    return Ok(line);

}

#[allow(dead_code)]
pub async fn send_server(stream:&mut TokioRustLSTlsStream<TokioTcpStream>,m:&str) -> Result<(),String> {
    if PRINT_MESSAGES && PRINT_DATA_MESSAGES{
        println!("{:?}",m);
    }
    match stream.write_all(&m.as_bytes()).await{
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{
            return Err(error("failed-send-send_result"));
        }
    }
}

#[allow(dead_code)]
pub fn secure_send_only(stream:&mut TlsStream<TcpStream>,m:String) -> Result<(),String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
    } else {
        let hold = c.split("\r\n").collect::<Vec<&str>>();
        if hold[hold.len() - 1].len() > 0{
            c.push_str("\r\n");
        }
    }
    if PRINT_MESSAGES && PRINT_DATA_MESSAGES{
        println!("{:?}",c);
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

#[allow(dead_code)]
pub fn secure_send(stream:&mut TlsStream<TcpStream>,m:String) -> Result<READ,String> {
    let mut c = m.clone();
    if c.contains("\r\n") == false {
        c.push_str("\r\n");
    }
    if PRINT_MESSAGES{
        println!("{:?}",c);
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

#[allow(dead_code)]
pub fn secure_read(stream:&mut TlsStream<TcpStream>) -> Result<READ,&'static str> {


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

    if PRINT_MESSAGES{
        println!("{:?}",line);
    }

    // println!("\n{:?}\n",line);

    let line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();

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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn parse(letter:&mut String,features:Vec<Feature>) -> Result<READ,&'static str>{
    // println!(">>>>>>>>>>>>>>> {:?}",letter);
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

#[allow(dead_code)]
pub fn cwd()->String{
    match std::env::current_dir(){
        Ok(v)=>{
            match v.as_path().to_str(){
                Some(f)=>{
                    f.to_string()
                },
                None=>{
                    String::new()
                }
            }
        },
        Err(_)=>{
            String::new()
        }
    }
}

fn parse_request(m:&String,r:&Regex) -> Result<Vec<String>,&'static str>{
    match r.captures(m){
        Some(r)=>{
            let mut collect = vec!();
            for cap in r.iter(){
                match cap{
                    Some(v)=>{
                        collect.push(v.as_str().to_string());
                    },
                    None=>{}
                }
            }
            return Ok(collect);
        },
        None=>{
            return Err("failed-regex-captures");
        }
    }
}

#[allow(dead_code)]
pub fn load_key(path:String) -> Result<Vec<PrivateKey>,&'static str>{

    let file:File;
    match File::open(path){
        Ok(f)=>{
            file = f;
        },
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    match rsa_private_keys(&mut BufReader::new(file)){
        Ok(vecs)=>{
            let mut collect:Vec<PrivateKey> = vec!();
            for vec in vecs{
                if vec.len() == 0{
                    return Err("empty_vec");
                } else {
                    collect.push(PrivateKey(vec));
                }
            }
            return Ok(collect);
        },
        Err(_)=>{
            return Err("failed-certs");
        }
    }

}

#[allow(dead_code)]
pub fn load_cert(path:String) -> Result<Vec<Certificate>,&'static str>{

    let file:File;
    match File::open(path){
        Ok(f)=>{
            file = f;
        },
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    match certs(&mut BufReader::new(file)){
        Ok(vecs)=>{
            let mut collect:Vec<Certificate> = vec!();
            for vec in vecs{
                if vec.len() == 0{
                    return Err("empty_vec");
                } else {
                    collect.push(Certificate(vec));
                }
            }
            return Ok(collect);
        },
        Err(_)=>{
            return Err("failed-certs");
        }
    }

}

#[allow(dead_code)]
pub async fn async_server_read_till(
    stream:&mut TokioRustLSTlsStream<TokioTcpStream>,
    end:&Vec<u8>,
    buffer:&mut Vec<u8>,
    max_size:usize
)->Result<String,&'static str>{

    let mut buffer_cursor = 0;

    loop {

        if buffer.len() >= max_size{
            buffer.clear();
            return Err("overflow");
        }

        let mut local_buffer = [0;512];
        match stream.read(&mut local_buffer).await{
            Ok(_v)=>{
                let mut hold = local_buffer.to_vec();
                let _ = hold.split_off(_v);
                buffer.append(&mut hold);
            },
            Err(_)=>{
                return Err("failed-read");
            }
        }

        match vector_in_vector(&buffer,end,buffer_cursor){
            Ok(found_at)=>{
                if found_at.0 == 0{
                    let _ = buffer.split_off(end.len());
                }
                let mut pending = buffer.split_off(found_at.1+1);
                let mut request = buffer.clone();
                buffer.clear();
                buffer.append(&mut pending);
                for _ in 0..end.len(){
                    request.remove(request.len()-1);
                }
                match String::from_utf8(request){
                    Ok(v)=>{
                        return Ok(v);
                    },
                    Err(_)=>{
                        return Err("invalid_request");
                    }
                }
            },
            Err(_)=>{}
        }

        if buffer.len() > 6{
            buffer_cursor = buffer.len() - 6;
        } else {
            buffer_cursor = 0;
        }

    }

}

#[allow(dead_code)]
fn vector_in_vector(v1:&Vec<u8>,v2:&Vec<u8>,start_cursor:usize)->Result<(usize,usize),()>{

    let mut v2_cursor:usize = 0;
    let mut found_some = false;
    let mut final_found = false;
    let mut v1_index = start_cursor;
    let mut start_index = 0;
    let mut end_index = 0;

    for n in start_cursor..v1.len(){
        let i = &v1[n];
        if !found_some{
            if i == &v2[0]{
                found_some = true;
                v2_cursor = 1;
                start_index = v1_index;
            }
        } else {
            if i != &v2[v2_cursor]{
                if i == &v2[0]{
                    found_some = true;
                    v2_cursor = 1;
                    start_index = v1_index;
                } else {
                    found_some = false;
                    v2_cursor = 0;
                }
            } else {
                if v2_cursor == v2.len() - 1{
                    final_found = true;
                    end_index = v1_index;
                    break;
                } else {
                    v2_cursor += 1;
                }
            }
        }
        v1_index += 1;
    }

    if final_found{
        Ok((start_index,end_index))
    } else {
        Err(())
    }

}