use crate::io;
use crate::client::{Action,Email,parse,resolve};
use std::net::TcpStream;
use native_tls::{TlsStream,TlsConnector};

pub fn send_mail(email:Email) -> Result<(),&'static str> {

    let actions:Vec<Action>;
    match parse::init(email.clone()){
        Ok(a)=>{
            actions = a;
        },
        Err(_)=>{
            return Err("failed-parse_mail");
        }
    }

    if email.to.contains("localhost"){
        let to_mail = email.to.clone();
        let email_split = to_mail.split("@").collect::<Vec<&str>>();
        match try_port(email_split[1].to_string(),&actions){
            Ok(_)=>{return Ok(())},
            Err(_)=>{return Err("failed-send_to_locahost");}
        }
    }

    let holders:resolve::RESV;
    match resolve::init(email.to.clone()){
        Ok(h)=>{
            holders = h;
        },
        Err(e)=>{
            println!("!!! failed-resolve-dns : {:?}",e);
            return Err("failed-resolve_dns");
        }
    }

    if holders.base.len() > 0{
        match try_address(holders.base,&actions){
            Ok(_)=>{return Ok(());},
            Err(_)=>{
                return Err("failed-try_address");
            }
        }
    } else {
        for runner in holders.pool.iter(){
            let mut addr = runner.exchange().to_utf8();
            if addr.as_bytes()[addr.len()-1] == ".".as_bytes()[0]{
                // addr.split_off(&addr.len()-1);
                addr.truncate(&addr.len()-1);   //changed from splitoff to truncate
            }
            match try_address(addr,&actions){
                Ok(_)=>{return Ok(());},
                Err(e)=>{
                    if e== "dont_continue"{
                        return Err("host_denied_mail");
                    }
                }
            }
        }
    }

    return Err("failed-not_reachable");

}

fn try_address(addr:String,actions:&Vec<Action>) -> Result<(),&'static str>{
    // let ports = ["25","465","587","2525"];
    let ports = ["25"];
    for port in ports.iter(){
        match try_port(format!("{}:{}",addr,port),&actions){
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                // println!("!!! email denied by the service");
                if e == "dont_continue"{return Err("dont_continue")}
            }
        }
    }
    return Err("failed-un_reacheble-on_ports");
}

fn try_port(addr:String,actions:&Vec<Action>) -> Result<(),&'static str>{

    let mut stream;
    match TcpStream::connect(addr.clone()) {
        Ok(r)=>{
            stream = r;
        },
        Err(e)=>{
            println!("error : {:?}",e);
            return Err("failed-start-connection");
        }
    }

    if false{
        match try_port_secure(addr,stream, actions){
            Ok(_)=>{
                return Ok(());
            },
            Err(_)=>{
                return Err("failed-starttls");
            }
        }
    }

    for action in actions {
        if action.io == "read" {
            match io::read(&mut stream){
                Ok(r)=>{
                    // println!("read action : {:?}, result : {:?}",action.tag,r);
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
                    Ok(_)=>{},
                    Err(_)=>{
                        println!("failed-write-execute-for => {:#?}",action);
                        return Err("failed-write-data");
                    }
                }
            } else {
                match io::send(&mut stream,action.cmd.to_string()){
                    Ok(r)=>{
                        // println!("{:?}",r);
                        if r.result == false {
                            // println!("action : {:?}",action);
                            return Err("failed-write-cmd-result");
                        } else {
                            if action.tag == "say_hello"{
                                // println!("write action : {:?}, result : {:?}",action.tag,r);
                                let mut can_tls = false;
                                for feature in r.features{
                                    if feature._type == "STARTTLS"{can_tls = true;}
                                }
                                if can_tls{
                                    match io::send(&mut stream,"STARTTLS".to_string()){
                                        Ok(r)=>{
                                            // println!("============ {:?}",r);
                                            if !r.result{return Err("failed-STARTTLS");} else {
                                                match try_port_secure(addr,stream, actions){
                                                    Ok(_)=>{
                                                        return Ok(());
                                                    },
                                                    Err(e)=>{
                                                        if e == "dont_continue" {
                                                            return Err("dont_continue");
                                                        }
                                                        println!("failed-start_tls : {:?}",e);
                                                        return Err("failed-starttls");
                                                    }
                                                }
                                            }
                                        },
                                        Err(_)=>{return Err("failed-STARTTLS")}
                                    }
                                }
                            }
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

fn try_port_secure(addr:String,base_stream:TcpStream,actions:&Vec<Action>) -> Result<(),&'static str>{

    // println!("=============== {:?}",addr);

    let addr_split = addr.split(":").collect::<Vec<&str>>();
    let only_addr = addr_split[0];

    let mut stream:TlsStream<TcpStream>;
    match TlsConnector::new(){
        Ok(connector)=>{
            match connector.connect(only_addr, base_stream){
                Ok(s)=>{stream = s;},
                Err(_)=>{return Err("failed-connect-tls_connector");}
            }
        },
        Err(_)=>{return Err("failed-start-tls_connector");}
    }

    // println!("=============== {:?}",addr);

    let mut actions_index = 0;

    for action in actions {
        // println!("{:?}",action);
        if actions_index > 0{
            // println!("{:?}",action);
            if action.io == "read" && actions_index > 0 {
                match io::secure_read(&mut stream){
                    Ok(r)=>{
                        // println!("read action : {:?}, result : {:?}",action.tag,r);
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
                // println!("action : {:?}",action);
                if action.cate == "data" {
                    match io::secure_send_only(&mut stream,action.cmd.to_string()){
                        Ok(_)=>{},
                        Err(_)=>{
                            println!("failed-write-execute-for => {:#?}",action);
                            return Err("failed-write-data");
                        }
                    }
                } else {
                    match io::secure_send(&mut stream,action.cmd.to_string()){
                        Ok(r)=>{
                            // println!("{:?}",r);
                            if r.result == false {
                                if r.code == 550{
                                    return Err("dont_continue");    //service denied the mail
                                }
                                println!("write failed : {:?}",r);
                                println!("action : {:?}",action);
                                return Err("failed-write-cmd-result");
                            }
                        },
                        Err(e)=>{
                            println!("{:?}",e);
                            println!("failed-write-execute-for => {:#?}",action);
                            return Err("failed-write-cmd");
                        }
                    }
                }
            }
        }
        actions_index += 1;
    }//loop

    return Ok(());

}
