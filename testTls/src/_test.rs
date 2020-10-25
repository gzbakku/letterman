use native_tls::{TlsConnector,TlsStream};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {

    match connect("mta5.am0.yahoodns.net","25"){
        Ok(_)=>{
            println!("connection complete");
        },
        Err(e)=>{
            println!("connection failed : {:?}",e);
        }
    }

}

fn connect(addr:&'static str,port:&'static str) -> Result<(),&'static str>{

    println!("addr : {:?} port : {:?}",addr,port);

    let with_port = format!("{}:{}",addr,port);
    let simpleStream:TcpStream;
    match TcpStream::connect(with_port.as_str()){
        Ok(s)=>{simpleStream = s;},
        Err(_)=>{return Err("failed-simple_connection");}
    }

    println!("simpleStream started : {:?}",simpleStream);

    let mut stream:TlsStream<TcpStream>;
    match TlsConnector::new(){
        Ok(connector)=>{
            match connector.connect(addr,simpleStream){
                Ok(s)=>{stream = s;},
                Err(e)=>{println!("{:?}",e);return Err("failed-connect-tls_connector");}
            }
        },
        Err(_)=>{return Err("failed-start-tls_connector");}
    }

    // let connector = TlsConnector::new().unwrap();
    // let streamBuilder = TcpStream::connect(addr).unwrap();
    // let mut stream = connector.connect(addr, streamBuilder).unwrap();

    stream.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));

    return Ok(());

}
