use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use std::io::{BufReader};
use rustls_pemfile::{certs,rsa_private_keys};
use std::fs::File;
use tokio_rustls::server::TlsStream as TokioRustLSTlsStream;
use tokio::net::TcpStream as TokioTcpStream;
use crate::common::{error};
use regex::Regex;

const PRINT_MESSAGES:bool = false;
const PRINT_DATA_MESSAGES:bool = false;

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