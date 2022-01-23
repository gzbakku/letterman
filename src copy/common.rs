use md5;
use std::time::{SystemTime,UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha256::digest as Sha256Digest;

#[allow(dead_code)]
pub fn sha256(v:String)->String{
    Sha256Digest(v)
}

#[allow(dead_code)]
pub fn random_string(size:usize)->String{
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    return rand_string;
}

#[allow(dead_code)]
pub fn error(e: &'static str) -> String {
    println!("!!! {:?}",e);
    return String::from(e);
}

#[allow(dead_code)]
pub fn log(m: &str){
    println!(">>> {:?}",m);
}

#[allow(dead_code)]
pub fn hash(v:String) -> String{
    format!("{:?}",md5::compute(v))
}

#[allow(dead_code)]
pub fn p_error(v:&'static str,print:bool){
    if print{
        format!("!!! {:?}",v);
    }
}

#[allow(dead_code)]
pub fn get_time() -> Result<u128,&'static str>{
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            return Ok(n.as_millis());
        },
        Err(_) => {
            return Err("failed-get_time");
        }
    }
}
