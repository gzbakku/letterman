use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use std::io::{BufReader};
use rustls_pemfile::{certs,rsa_private_keys};
use std::fs::File;
use tokio_rustls::server::TlsStream as TokioRustLSTlsStream;
use tokio::net::TcpStream as TokioTcpStream;
use crate::common::{error};
use regex::Regex;
use tokio::time::timeout;
use std::time::Duration;

const TIMEOUT_DURATION:u64 = 10;
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
    match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.write_all(&m.as_bytes())).await{
        Ok(r)=>{
            match r{
                Ok(_)=>{
                    return Ok(());
                },
                Err(_)=>{
                    return Err(error("failed-send-send_result"));
                }
            }
        },
        Err(_)=>{
            return Err(error("timeout"));
        }
    }
}

pub async fn read_server(stream:&mut TokioRustLSTlsStream<TokioTcpStream>,r:&Regex) -> Result<Vec<String>,&'static str> {

    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
            Ok(r)=>{
                match r{
                    Ok(len)=>{
                        for i in 0..len{collect.push(buff[i].clone());}
                        if len < 5000 {break;}
                    },
                    Err(e)=>{
                        println!("failed read bytes : {:?}",e);
                        return Err("failed read bytes");
                    }
                }
            },
            Err(_)=>{return Err("timeout");}
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

    // println!("{:?}",end);

    let mut buffer_cursor = 0;

    loop {

        if buffer.len() >= max_size{
            println!("!!! overflow");
            buffer.clear();
            return Err("overflow");
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

        let mut local_buffer = [0;512];
        
        match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut local_buffer)).await{
            Ok(r)=>{
                match r{
                    Ok(_v)=>{
                        let mut hold = local_buffer.to_vec();
                        let _ = hold.split_off(_v);
                        buffer.append(&mut hold);
                    },
                    Err(_)=>{
                        return Err("failed-read");
                    }
                }
            },
            Err(_)=>{
                return Err("timeout");
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

        if buffer.len() > 8{
            buffer_cursor = buffer.len() - 8;
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

pub async fn read_server_till_end(
    stream:&mut TokioRustLSTlsStream<TokioTcpStream>,
    buffer:&mut Vec<u8>,
    replace:&str
) -> Result<String,&'static str> {

    // let mut collect = Vec::new();
    // let mut buff = [0; 5000];

    loop {
        let mut local_buffer = [0;512];
        match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut local_buffer)).await{
            Ok(r)=>{
                match r{
                    Ok(_v)=>{
                        let mut hold = local_buffer.to_vec();
                        let _ = hold.split_off(_v);
                        buffer.append(&mut hold);
                        if _v < 512{
                            break;
                        }
                    },
                    Err(_)=>{
                        return Err("failed-read");
                    }
                }
            },
            Err(_)=>{
                return Err("timeout");
            }
        }
    }

    match String::from_utf8(buffer.to_vec()) {
        Ok(mut result)=>{
            buffer.clear();
            if replace.len() == 0{
                return Ok(result);
            } else {
                while result.contains(replace){
                    result = result.replace(replace,"");
                }
                return Ok(result); 
            }
        },
        Err(_)=>{
            return Err("failed-parse_byte_array-to_string-read");
        }
    }

}