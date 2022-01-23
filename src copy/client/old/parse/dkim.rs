use crate::client_old::{Email,Action};
use openssl::sha::Sha256;
use base64;

use openssl::sign::Signer;
use openssl::pkey::{PKey,Private};
use openssl::hash::MessageDigest;

pub fn init(email:&Email,domain:String,message_id:String,body:Vec<Action>) -> Result<String,&'static str>{

    let mut compile = String::new();

    //********************
    //dkim headers

    compile = add_field(compile,"v".to_string(),"1".to_string());
    compile = add_field(compile,"a".to_string(),"rsa-sha256".to_string());
    compile = add_field(compile,"c".to_string(),"relaxed/relaxed".to_string());
    compile = add_field(compile,"d".to_string(),domain);
    compile = add_field(compile,"q".to_string(),"dns/txt".to_string());
    compile = add_field(compile,"s".to_string(),email.dkim_selector.clone());

    //********************
    //cauculate header hash

    let mut _concantat_header = String::new();
    _concantat_header = join_header(_concantat_header,"from",format!("{} <{}>",email.name,email.from));
    _concantat_header = join_header(_concantat_header,"to",format!("{}",email.to));
    _concantat_header = join_header(_concantat_header,"subject",format!("{}",email.subject));
    _concantat_header = join_header(_concantat_header,"date",format!("{}",email.date));
    _concantat_header = join_header(_concantat_header,"message-id",format!("{}",message_id));

    //********************
    //cauculate body hash

    let mut _compile_body = String::new();
    for i in body{
        let v = clean_body(i.cmd);
        _compile_body.push_str(&v);
    }

    // println!("\n{:?}\n",_compile_body);

    let body_hash = hash_sha256_encoded(_compile_body);
    compile = add_field(compile,"bh".to_string(),body_hash);
    compile = add_field(compile,"h".to_string(),"from:to:subject:date:message-id".to_string());
    compile = add_field(compile,"b".to_string(),"".to_string());

    //********************
    //cauculate dkim signature

    let final_build = format!("{}dkim-signature:{}",_concantat_header,compile);

    // println!("\n\n final_build : {:?}\n\n",final_build);

    match sign_here(email.private_key.clone(),final_build.into_bytes()){
        Ok(dkim_signature)=>{
            compile = compile.replace("b=",&format!("b={}",dkim_signature));
        },
        Err(_)=>{
            return Err("failed-sign");
        }
    }

    return Ok(compile);

}

fn join_header(base:String,tag:&'static str,val:String) -> String{
    let mut h = base.to_string();
    h.push_str(&format!("{}:{}\r\n",tag,clean_string(val)));
    return h;
}

fn add_field(base_in:String,tag:String,value:String) -> String {
    let mut base = base_in;
    if base.len() > 0{
        base.push_str(&"; ".to_string());
    }
    base.push_str(&format!("{}={}",tag,value));
    return base;
}

fn clean_body(v:String) -> String{
    if v.len() == 0{
        return format!("\r\n");
    }
    if v.contains("\r\n"){
        let hold = v.split("\r\n").collect::<Vec<&str>>();
        let mut compile = String::new();
        for i in hold{
            if i.len() > 0{
                compile.push_str(&format!("{}\r\n",clean_string(i.to_string())));
            }
        }
        return compile;
    } else {
        return format!("{}\r\n",clean_string(v));
    }
}

fn clean_string(v:String) -> String{
    let hold = v.split(" ").collect::<Vec<&str>>();
    let mut collect = String::new();
    if collect.contains("\t"){
        while collect.contains("\t"){
            collect = collect.replace("\t"," ");
        }
    }
    if collect.contains("\r\n"){
        while collect.contains("\r\n"){
            collect = collect.replace("\r\n","");
        }
    }
    for i in hold{
        if i.len() > 0{
            if collect.len() > 0{
                collect.push_str(&format!(" "));
            }
            collect.push_str(&format!("{}",i));
        }
    }
    if collect.len() == 0{
        collect = " ".to_string();
    }
    return collect;
}

fn hash_sha256_encoded(v:String) -> String{
    let mut hasher = Sha256::new();
    hasher.update(&v.into_bytes());
    let hash = hasher.finish();
    let encode = base64::encode(&hash);
    return encode;
}

#[allow(dead_code)]
fn hash_sha256(v:String) -> Vec<u8>{
    let mut hasher = Sha256::new();
    hasher.update(&v.into_bytes());
    let hash = hasher.finish();
    return hash.to_vec();
}

fn sign_here(key:String,val:Vec<u8>) -> Result<String,&'static str>{

    let private_key:PKey<Private>;
    match PKey::private_key_from_pem(&key.into_bytes()){
        Ok(k)=>{
            private_key = k;
        },
        Err(e)=>{
            println!("!!! failed-parse_private_key => {:?}",e);
            return Err("failed-invalid_key");
        }
    }

    let encoded_signature:String;
    match Signer::new(MessageDigest::sha256(), &private_key){
        Ok(mut signer)=>{
            match signer.update(&val){
                Ok(_)=>{
                    match signer.sign_to_vec(){
                        Ok(signature)=>{
                            encoded_signature = base64::encode(&signature);
                        },
                        Err(_)=>{return Err("failed-sign");}
                    }
                },
                Err(_)=>{return Err("failed-insert_data_into_signer");}
            }
        },
        Err(_)=>{return Err("failed-initiate_signer");}
    }

    return Ok(encoded_signature);

}
