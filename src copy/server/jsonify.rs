use crate::server::config::{Email};
use letterman_email_body_parser::{EmailBody,ContentDecoded,Part};
use json::{object,JsonValue,stringify};
use std::time::{SystemTime};
use crate::common::{sha256,random_string};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
// use tokio::sync::RwLock;
// use std::sync::Arc;
// use crate::server::config::{ServerInfo};

pub async fn init(
    // info:&Arc<RwLock<ServerInfo>>,
    email:Email,
    body:EmailBody,
    dir:&String
)->Result<Vec<u8>,&'static str>{

    // let info_lock = info.read().await;
    let mut body = body;

    let time:u128;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            time = n.as_millis();
        },
        Err(_) => {
            return Err("failed-systime");
        }
    }

    // println!("{:?}",body.headers);

    let from:String;
    let to:String;
    match body.headers.get("from"){
        Some(v)=>{from = v.clone();},
        None=>{return Err("failed-get-from");}
    }
    match body.headers.get("to"){
        Some(v)=>{to = v.clone();},
        None=>{return Err("failed-get-to");}
    }

    // println!("from : {:?}",from);
    // println!("to : {:?}",to);
    // match info_lock.regex.email.captures(from){
    //     Some(captures)=>{
    //         match captures.get(0){
    //             Some()=>{},
    //             None=>{}
    //         }
    //     },
    //     None=>{}
    // }

    let random = random_string(7);
    let id_string = format!("from:{:?} to:{:?} time:{:?} random:{:?}",from,to,time,random);
    let id_hash = sha256(id_string);
    let mut to_write = Vec::new();
    let mut build = object!{
        "id":id_hash.clone(),
        "sender":email.sender,
        "receivers":email.receivers,
        "headers":{},
        "body":[],
        "attachments":[]
    };

    for (key,value) in body.headers.iter(){
        build["headers"][key] = JsonValue::String(value.to_string());
    }

    loop{
        if body.body.len() == 0{break;}
        // println!("removing body");
        let part = body.body.remove(0);
        match part.decoded{
            ContentDecoded::String(v)=>{
                match build["body"].push(object!{
                    "type":"string",
                    "value":v
                }){Ok(_)=>{},Err(_)=>{return Err("failed-build_string_object");}}
            },
            ContentDecoded::Html(v)=>{
                match build["body"].push(object!{
                    "type":"html",
                    "value":v
                }){Ok(_)=>{},Err(_)=>{return Err("failed-build_html_object");}}
            },
            _=>{
                let (object,(file_name,value)) = parse_file(&id_hash,part);
                match build["body"].push(object!{
                    "type":"file",
                    "value":object
                }){Ok(_)=>{
                    to_write.push((file_name,value));
                },Err(_)=>{return Err("failed-build_file_object");}}
            }
        }
    }

    loop{
        if body.attachments.len() == 0{break;}
        // println!("removing attachments");
        let part = body.attachments.remove(0);
        let (object,(file_name,value)) = parse_file(&id_hash,part);
        match build["attachments"].push(object!{
            "type":"file",
            "value":object
        }){Ok(_)=>{
            to_write.push((file_name,value));
        },Err(_)=>{return Err("failed-build_file_object");}}
    }

    loop{
        if to_write.len() == 0{break;}
        // println!("removing to_write");
        let (file_name,value) = to_write.remove(0);
        let path = format!("{}{}",dir,file_name);
        let value_as_bytes:Vec<u8>;
        match value{
            ContentDecoded::Base64(v)=>{value_as_bytes = v;},
            ContentDecoded::Qp(v)=>{value_as_bytes = v;},
            ContentDecoded::Html(v)=>{value_as_bytes = v.as_bytes().to_vec();},
            ContentDecoded::String(v)=>{value_as_bytes = v.as_bytes().to_vec();},
            ContentDecoded::None=>{
                return Err("not_writable");
            }
        }
        match write_file(path, value_as_bytes).await{
            Ok(_)=>{},
            Err(_)=>{
                return Err("failed-write_file");
            }
        }
    }

    // println!("jsonify done");

    return Ok(stringify(build).as_bytes().to_vec());

}

fn parse_file(id:&String,part:Part)->(JsonValue,(String,ContentDecoded)){

    let mut extension = String::new();
    if part.content_type.0.len() > 0{
        if part.content_type.0.contains("/"){
            let hold:Vec<&str> = part.content_type.0.split("/").collect();
            extension = hold[hold.len()-1].to_string();
        } else {
            extension = part.content_type.0.to_string();
        }
    }

    let name:String;
    match part.content_type.1.get("name"){
        Some(v)=>{
            name = v.to_string();
        },
        None=>{name = random_string(10);}
    }
    let random = random_string(5);
    let name_hash = sha256(format!("{}{}",name.clone(),random));
    let mut file_name = format!("{}_{}",id,name_hash);
    if extension.len() > 0{
        file_name = format!("{}.{}",file_name,extension);
    }
    let mut build = object!{
        name:name,
        hash:name_hash,
        file_name:file_name.clone(),
        features:{},
        content_features:{}
    };
    for (key,value) in part.content_type.1.iter(){
        build["content_features"][key] = JsonValue::String(value.to_string());
    }
    for (key,value) in part.content_features.iter(){
        build["features"][key] = JsonValue::String(value.to_string());
    }

    return (
        build,
        (
            file_name,
            part.decoded
        )
    );

}

async fn write_file(path:String,data:Vec<u8>)->Result<(),&'static str>{

    let mut file:File;
    match File::create(path).await{
        Ok(v)=>{file = v;},
        Err(_)=>{return Err("failed-create_file");}
    }

    match file.write(&data).await{
        Ok(_)=>{
            return Ok(());
        },
        Err(_)=>{return Err("failed-create_file");}
    }

}