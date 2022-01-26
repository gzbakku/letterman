use crate::server::config::{Email};
use letterman_email_body_parser::{EmailBody,ContentDecoded,Part};
use json::{object,JsonValue,stringify};
use std::time::{SystemTime};
use crate::common::{random_string};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn init(
    email:Email,
    mut body:EmailBody,
    dir:&String
)->Result<Vec<u8>,&'static str>{

    let time:u128;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            time = n.as_millis();
        },
        Err(_) => {
            return Err("failed-systime");
        }
    }

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
    let mut dkim = String::new();
    match body.headers.get("dkim-signature"){
        Some(v)=>{dkim = v.clone();},
        None=>{}
    }

    let random = random_string(7);
    let id_string = format!("from:{:?} to:{:?} time:{:?} random:{:?} dkim:{:?}",from,to,time,random,dkim);
    let id_hash = hash_md5(id_string);
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

    return Ok(stringify(build).as_bytes().to_vec());

}

fn parse_file(id:&String,part:Part)->(JsonValue,(String,ContentDecoded)){

    //----------------------
    //get file name
    //----------------------
    let mut name = String::new();
    if part.content_features.contains_key("name"){
        match part.content_features.get("name"){
            Some(v)=>{name = v.to_string();},
            None=>{}
        }
    } else if part.content_features.contains_key("file_name"){
        match part.content_features.get("file_name"){
            Some(v)=>{name = v.to_string();},
            None=>{}
        }
    } else if part.content_type.1.contains_key("name"){
        match part.content_type.1.get("name"){
            Some(v)=>{name = v.to_string();},
            None=>{}
        }
    } else if part.content_type.1.contains_key("file_name"){
        match part.content_type.1.get("file_name"){
            Some(v)=>{name = v.to_string();},
            None=>{}
        }
    }
    if name.len() == 0 {name = random_string(3);}
    let file_name = format!("{}_{}_{}",id,random_string(2),name);

    let mut build = object!{
        name:name,
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

fn hash_md5(v:String)->String{
    format!("{:?}",md5::compute(v))
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