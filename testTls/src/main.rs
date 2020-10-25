use native_tls::{TlsConnector,TlsStream};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

mod worker;

fn main() {

    use native_tls::TlsConnector;
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let connector = TlsConnector::new().unwrap();

    let mut stream = TcpStream::connect("mta5.am0.yahoodns.net:25").unwrap();

    stream.set_read_timeout(Some(Duration::from_secs(1)));

    // let mut stream = connector.connect("localhost", stream).unwrap();


    worker::read_message(&mut stream);

    worker::write_message(&mut stream,"EHLO silvergram.in");

    worker::read_message(&mut stream);

    worker::write_message(&mut stream,"STARTTLS");

    worker::read_message(&mut stream);

    let mut stream = connector.connect("mta5.am0.yahoodns.net", stream).unwrap();

    worker::write_secure(&mut stream,"EHLO silvergram.in");
    worker::read_secure(&mut stream);

    worker::write_secure(&mut stream,"MAIL FROM:<akku@silvergram.in>");
    worker::read_secure(&mut stream);

    // let mut res = vec![];
    // match stream.read_to_end(&mut res){
    //     Ok(p)=>{
    //         println!("success read_to_end : {:?}",p);
    //     },
    //     Err(e)=>{
    //         println!("failed read_to_end : {:?}",e);
    //     }
    // }
    // println!("{}", String::from_utf8_lossy(&res));

    // println!("{:?}",res);

    // match stream.write_all(b"EHLO silvergram.in\r\n"){
    //     Ok(p)=>{
    //         println!("success write_all : {:?}",p);
    //     },
    //     Err(e)=>{
    //         println!("failed write_all : {:?}",e);
    //     }
    // }





    // println!("{}", String::from_utf8_lossy(&res));

}
