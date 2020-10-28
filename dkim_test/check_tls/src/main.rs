use openssl::sha::Sha256;
use base64;
use openssl::sign::Signer;
use openssl::pkey::{PKey,Private};
use openssl::hash::MessageDigest;

fn main() {

    let body = "first message\r\nsecond message\r\nthird message\r\n".to_string();
    let bh = "HUO5tf/t5nm+UBl5Su6jGDxjrAIHzK4ASO6wJGJ/0oc".to_string();

    println!("\n\n", );

    println!("{:?}",hash_sha256(body));
    println!("{:?}",base64::decode(&bh).unwrap());

    println!("\n\n", );

    //--------------------------------
    //the big sig

    let sig = "SImGo8hiQ7ULzsXfKzcesPlVQkGOiVrjDe+cVZx4SoxTIP/mlcP+LFzVP1KzBdCL90KH49JmqzKdqz4Srge2LM/CtrLCjLeBZHNBaI/j6KP/yiSbcmbQE/0VGI0UZRvklfB2e0HCGaoboKGNjQJk4jbaOioiiuyDQjRU/mZTXXp0laE+1UcUidOc7yA6D6V7gA2N/bCLxnm4hy/gtbJBXxyjHXslqsLlnADDcvXc63Y4O3twxbod/G8fbB7UjngP8Bz/uMINrxi1Uh4di0hkxp3UMVVpjQ8u1+qDMBtKDHeL9AW2kKW8dD8G/AZNnKVyp4pOQoAEy0Swlcf1PKNqLQ==".to_string();

    let dkim = "v=1; a=rsa-sha256; c=relaxed/relaxed; d=silvergram.in; q=dns/txt; s=dkim; bh=HUO5tf/t5nm+UBl5Su6jGDxjrAIHzK4ASO6wJGJ/0oc=; h=from:subject:date:message-id:to:mime-version:content-type:content-transfer-encoding; b=".to_string();

    let mut header_build = String::new();
    header_build = join_header(header_build,"from","gzbakku@silvergram.in");
    header_build = join_header(header_build,"subject","Hello world");
    header_build = join_header(header_build,"date","Wed, 28 Oct 2020 07:34:22 +0000");
    header_build = join_header(header_build,"message-id","<b02531d6-3863-5f79-4eb4-2fa467219c70@silvergram.in>");
    header_build = join_header(header_build,"to","9mvxra2pyfknnz@dkimvalidator.com");
    header_build = join_header(header_build,"mime-version","1.0");
    header_build = join_header(header_build,"content-type","text/plain; charset=utf-8");
    header_build = join_header(header_build,"content-transfer-encoding","7bit");

    let final_build = format!("{}dkim-signature:{}",header_build,dkim);

    println!("\n\n{}\n\n",final_build);

    // println!("{:?}",get_key());

    println!("{:?}",sig);
    println!("\n\n{:?}\n\n",sign_here(get_key(),(final_build).into_bytes()).unwrap());

}

fn join_header(base:String,tag:&'static str,val:&'static str) -> String{
    let mut h = base.to_string();
    h.push_str(&format!("{}:{}\r\n",tag,val.to_string()));
    return h;
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

fn get_key() -> String{

String::from("provide your private key here and dont add extract spaces between lines like spaces or tabs")

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
