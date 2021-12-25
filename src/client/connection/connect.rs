use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::{ResolverConfig,ResolverOpts};
use trust_dns_resolver::proto::rr::rdata::mx::MX;
use crate::client::Connection;
use trust_dns_resolver::{TokioConnection,TokioConnectionProvider};
use std::collections::HashMap;
use tokio::net::TcpStream;
use native_tls::TlsConnector;
use tokio_native_tls::TlsConnector as TokioNativeTlsConnector;
use tokio_native_tls::TlsStream;

const PORTS: &'static [u32] = &[587,2525,25];

pub enum Connected{
    Secure(TlsStream<TcpStream>),InSecure(TcpStream)
}

pub async fn init(connection:&mut Connection)->Result<(Connected,u32),&'static str>{

    let mut mx_records:Vec<String>;
    match get_mx_records(connection.domain.to_string()).await{
        Ok(v)=>{mx_records = v;},
        Err(_)=>{
            return Err("failed-get_mx_records");
        }
    }

    loop{
        if mx_records.len() == 0{
            return Err("no_working_domain");
        }
        let domain = mx_records.remove(0);
        for port in PORTS{
            match start_connection(domain.clone(), port).await{
                Ok(v)=>{
                    // println!("{} {} => ok",&domain, &port);
                    return Ok((v,port.clone()));
                },
                Err(_e)=>{
                    // println!("{} {} => {:?}",&domain, &port, _e);
                }
            }
        }
    }

}

async fn start_connection(domain:String,port:&u32)->Result<Connected,&'static str>{

    let address = format!("{}:{}",domain,port);
    let stream:TcpStream;
    match TcpStream::connect(&address).await{
        Ok(v)=>{stream = v;},
        Err(_)=>{
            return Err("failed-start-TcpStream");
        }
    }

    match TlsConnector::builder()
    .danger_accept_invalid_certs(true)
    .build()
    {
        Ok(base)=>{
            let connector = TokioNativeTlsConnector::from(base);
            match connector.connect(&domain, stream).await{
                Ok(s)=>{
                    return Ok(Connected::Secure(s));
                },
                Err(_e)=>{
                    // println!("failed-connect-tls_connector : {:?}",_e);
                    match TcpStream::connect(&address).await{
                        Ok(v)=>{return Ok(Connected::InSecure(v));},
                        Err(_)=>{
                            return Err("failed-start-TcpStream-unsecure");
                        }
                    }
                    // return Err("failed-connect-tls_connector");
                }
            }
        },
        Err(_)=>{
            println!("tls connection failed");
            return Err("failed-start-tls_connector");
        }
    }

}

pub async fn start_tls(connection:Connected,domain:String,port:&u32)->Result<Connected,&'static str>{
    match connection{
        Connected::Secure(v)=>{return Ok(Connected::Secure(v));},
        Connected::InSecure(stream)=>{
            let address = format!("{}:{}",domain,port);
            match TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            {
                Ok(base)=>{
                    let connector = TokioNativeTlsConnector::from(base);
                    match connector.connect(&domain, stream).await{
                        Ok(s)=>{
                            return Ok(Connected::Secure(s));
                        },
                        Err(_e)=>{
                            match TcpStream::connect(&address).await{
                                Ok(v)=>{return Ok(Connected::InSecure(v));},
                                Err(_)=>{return Err("failed-connect-tls_connector");}
                            }
                        }
                    }
                },
                Err(_)=>{
                    match TcpStream::connect(&address).await{
                        Ok(v)=>{return Ok(Connected::InSecure(v));},
                        Err(_)=>{return Err("failed-start-TcpStream-unsecure");}
                    }
                }
            }
        }
    }
}

pub async fn get_mx_records(domain:String)->Result<Vec<String>,&'static str>{

    if domain.contains("localhost"){
        return Ok(vec!["127.0.0.1".to_string()]);
    }

    let resolver:AsyncResolver<TokioConnection,TokioConnectionProvider>;
    match AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()){
        Ok(v)=>{
            resolver = v;
        },
        Err(_)=>{
            return Err("failed-start-resolver");
        }
    }

    let mut map:HashMap<u16,String> = HashMap::new();
    let mut collect:Vec<u16> = Vec::new();

    match resolver.mx_lookup(domain).await{
        Ok(lookup)=>{
            for a in lookup.iter(){
                collect.push(a.preference());
                map.insert(a.preference(),parse_domain(&a));
            }
        },
        Err(_e)=>{
            return Err("failed-lookup");
        }
    }

    let mut sorted = Vec::new();
    collect.sort();
    loop{
        if collect.len() == 0{break;}
        let key = collect.remove(0);
        match map.remove(&key){
            Some(v)=>{
                sorted.push(v);
            },
            None=>{}
        }
    }

    // println!("{:?}",sorted);

    return Ok(sorted);

    // return Err("failed-lookup");

}

fn parse_domain(m:&MX) -> String{
    let mut addr = m.exchange().to_utf8();
    if addr.as_bytes()[addr.len()-1] == ".".as_bytes()[0]{
        addr.truncate(&addr.len()-1);   //changed from splitoff to truncate
    }
    return addr.to_string();
}