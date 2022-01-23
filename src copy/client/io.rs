use tokio::io::{AsyncReadExt,AsyncWriteExt};
use regex::Regex;
use tokio::fs::File;
use new_mime_guess::MimeGuess;
use base64::encode as Base64Encode;
use crate::client::connection::{Features,Connected};
use tokio::time::timeout;
use std::time::Duration;

const TIMEOUT_DURATION:u64 = 10;
const PRINT_MESSAGES:bool = false;

#[derive(Debug)]
pub struct READ {
    pub result:bool,
    pub code:u16,
    pub message:String
}

pub async fn read_file(path:String)->Result<(String,String,String),&'static str>{

    let mut file:File;
    match File::open(&path).await{
        Ok(f)=>{file = f;},
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer).await{
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-read_file");
        }
    }

    let encoded = Base64Encode(buffer);
    let mime:String;
    match MimeGuess::from_path(path.to_string()).first(){
        Some(name)=>{
            mime = format!("{}/{}",name.type_(),name.subtype());
        },
        None=>{
            mime = "application/octet-stream".to_string();
        }
    }

    let mut path = path;
    while path.contains("\\"){
        path = path.replace("\\","/");
    }
    let mut hold:Vec<&str> = path.split("/").collect();
    let file_name = hold.remove(hold.len()-1).to_string();

    return Ok((file_name,mime,encoded));

}

// match timeout(Duration::from_secs(TIMEOUT_DURATION)).await{
//     Ok(r)=>{
//         match r{
//             Ok(_)=>{},
//             Err(_)=>{}
//         }
//     },
//     Err(_)=>{
//         return Err("timeout");
//     }
// }

pub async fn secure_send_with_response(connected:&mut Connected,m:String) -> Result<READ,&'static str> {
    match secure_send(connected, m).await{
        Ok(_)=>{},
        Err(_)=>{return Err("failed-send-send_result");}
    }
    match secure_read(connected).await{
        Ok(response)=>{return Ok(response);},
        Err(_)=>{return Err("failed-send-send_result");}
    }
}

pub async fn secure_send_with_features(connected:&mut Connected,m:String) -> Result<Features,&'static str> {
    match secure_send(connected, m).await{
        Ok(_)=>{},
        Err(_)=>{return Err("failed-send-send_result");}
    }
    match secure_read_features(connected).await{
        Ok(features)=>{return Ok(features);},
        Err(_)=>{return Err("failed-send-send_result");}
    }
}

pub async fn secure_send(connected:&mut Connected,m:String) -> Result<(),&'static str> {
    if PRINT_MESSAGES{
        println!("{}",m);
    }
    match connected{
        Connected::Secure(ref mut stream)=>{
            match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.write_all(&m.as_bytes())).await{
                Ok(r)=>{
                    match r{
                        Ok(_)=>{return Ok(());},
                        Err(_)=>{return Err("failed-send-send_result");}
                    }
                },
                Err(_)=>{
                    return Err("timeout");
                }
            }
        },
        Connected::InSecure(ref mut stream)=>{
            match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.write_all(&m.as_bytes())).await{
                Ok(r)=>{
                    match r{
                        Ok(_)=>{return Ok(());},
                        Err(_)=>{return Err("failed-send-send_result");}
                    }
                },
                Err(_)=>{
                    return Err("timeout");
                }
            }
        }
    }
}

pub async fn secure_read(connected:&mut Connected) -> Result<READ,&'static str> {

    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match connected{
            Connected::Secure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
            },
            Connected::InSecure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
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

    let mut line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();
    // println!("\n\n{:?} {:?}\n\n",line_vec,line_vec.len());
    if line_vec.len() != 2 || line_vec[1].len() > 0 {
        return Err("invalid_message-multi_line_end");
    }

    match parse(&mut line_vec.remove(0).to_string()){
        Ok(p)=>{
            return Ok(p);
        },
        Err(_e)=>{
            println!("!!! failed-parse : {:?}",_e);
            return Err("failed-parse_response");
        }
    }

}

pub async fn secure_read_qued(connected:&mut Connected) -> Result<Vec<READ>,&'static str> {


    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match connected{
            Connected::Secure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
            },
            Connected::InSecure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
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

    let mut line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();
    let mut collect = Vec::new();
    loop{
        if line_vec.len() == 0{
            break;
        }
        let mut line = line_vec.remove(0).to_string();
        if line.len() > 0{
            match parse(&mut line){
                Ok(p)=>{
                    collect.push(p);
                },
                Err(_e)=>{
                    println!("!!! failed-parse_response : {:?}",_e);
                    return Err("failed-parse_response");
                }
            }
        }
    }

    return Ok(collect);

}

pub async fn secure_read_features(connected:&mut Connected) -> Result<Features,&'static str> {


    let mut collect = Vec::new();
    let mut buff = [0; 5000];

    loop {
        match connected{
            Connected::Secure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
            },
            Connected::InSecure(ref mut stream)=>{
                match timeout(Duration::from_secs(TIMEOUT_DURATION),stream.read(&mut buff)).await{
                    Ok(r)=>{
                        match r{
                            Ok(len)=>{
                                for i in 0..len{collect.push(buff[i].clone());}
                                if len < 5000 {break;}
                            },
                            Err(_)=>{
                                return Err("failed read 10 bytes");
                            }
                        }
                    },
                    Err(_)=>{
                        return Err("timeout");
                    }
                }
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

    let mut line_vec: Vec<&str> = line.split("\r\n").collect::<Vec<&str>>();
    let mut collect = Vec::new();
    loop{
        if line_vec.len() == 0{
            break;
        }
        let mut line = line_vec.remove(0).to_string();
        if line.len() > 0{
            match parse(&mut line){
                Ok(p)=>{
                    collect.push(p);
                },
                Err(_e)=>{
                    println!("!!! failed-parse_response : {:?}",_e);
                    return Err("failed-parse_response");
                }
            }
        }
    }

    match parse_features(&mut collect){
        Ok(v)=>{
            return Ok(v);
        },
        Err(_e)=>{
            println!("!!! failed-parse_features : {:?}",_e);
            return Err("failed-parse_features");
        }
    }

}

fn parse_features(c:&mut Vec<READ>)->Result<Features,&'static str>{

    let mut build = Features::default();
    build.result = true;
    loop{
        if c.len() == 0{
            break;
        }
        let hold = c.remove(0);
        if !hold.result{
            build.result = false;
            return Ok(build);
            // return Err("invalid_flag");
        }
        if hold.message.contains("SIZE"){
            match Regex::new(r#"SIZE\s([\d]+)"#){
                Ok(regex)=>{
                    match regex.captures(&hold.message){
                        Some(captures)=>{
                            match captures.get(1){
                                Some(size_str)=>{
                                    // println!("\n>>> size_str : {:?}\n",size_str.as_str());
                                    match size_str.as_str().to_string().parse::<u64>(){
                                        Ok(size)=>{
                                            build.limit_size = true;
                                            build.size = size;
                                        },
                                        Err(_)=>{
                                            return Err("failed-parse-size-u64");
                                        }
                                    }
                                },
                                None=>{
                                    return Err("not_found-size");
                                }
                            }
                        },
                        None=>{
                            return Err("invalid-size");
                        }
                    }
                },
                Err(_)=>{
                    return Err("failed-make_regex");
                }
            }
        } else
        if hold.message.contains("PIPELINING"){
            build.pipeline = true;
        } else
        if hold.message.contains("STARTTLS"){
            build.start_tls = true;
        }else
        if hold.message.contains("CHUNKING"){
            build.chunking = true;
        } else
        if hold.message.contains("HELP"){
            build.help = true;
        }
    }

    return Ok(build);

}

fn parse(letter:&mut String) -> Result<READ,&'static str>{
    if letter.len() <= 4 {
        return Err("empty message");
    }
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

// #[allow(dead_code)]
// pub async fn write_file(data:String)->Result<(),&'static str>{
//     let mut file:File;
//     match File::create("./sldv_alt_atch_pipe.txt").await{
//         Ok(f)=>{file = f;},
//         Err(_)=>{
//             return Err("failed-open_file");
//         }
//     }
//     match file.write(data.as_bytes()).await{
//         Ok(_)=>{},
//         Err(_)=>{
//             return Err("failed-write_file");
//         }
//     }
//     return Ok(());
// }