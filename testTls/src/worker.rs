use native_tls::{TlsConnector,TlsStream};
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn read_secure(stream:&mut TlsStream<TcpStream>){
    // let mut res = vec![];
    // match stream.read_to_end(&mut res){
    //     Ok(p)=>{
    //         println!("success read_secure : {:?}",p);
    //     },
    //     Err(e)=>{
    //         println!("failed read_secure : {:?}",e);
    //     }
    // }
    // println!("{}", String::from_utf8_lossy(&res));

    let mut collect = Vec::new();
    let mut buff = [0; 5000];
    loop {
        match stream.read(&mut buff) {
            Ok(len)=>{
                for i in 0..len{collect.push(buff[i].clone());}
                if len > 0 && len < 5000 {break;}
            },
            Err(_)=>{
                return;
            }
        }
    }
    match String::from_utf8(collect) {
        Ok(result)=>{
            println!("{:?}",result);
        },
        Err(_)=>{}
    }

}

pub fn write_secure(stream:&mut TlsStream<TcpStream>,m:&'static str){
    match stream.write_all(&format!("{}\r\n",m).into_bytes()){
        Ok(p)=>{
            println!("success write_secure : {:?}",p);
        },
        Err(e)=>{
            println!("failed write_secure : {:?}",e);
        }
    }
}

pub fn read_message(stream:&mut TcpStream){
    let mut collect = Vec::new();
    let mut buff = [0; 5000];
    loop {
        match stream.read(&mut buff) {
            Ok(len)=>{
                for i in 0..len{collect.push(buff[i].clone());}
                if len > 0 && len < 5000 {break;}
            },
            Err(_)=>{
                return;
            }
        }
    }
    match String::from_utf8(collect) {
        Ok(result)=>{
            println!("{:?}",result);
        },
        Err(_)=>{}
    }
}

pub fn write_message(stream:&mut TcpStream,m:&'static str){
    match stream.write_all(&format!("{}\r\n",m).into_bytes()){
        Ok(p)=>{
            println!("success write_all : {:?}",p);
        },
        Err(e)=>{
            println!("failed write_all : {:?}",e);
        }
    }
}
