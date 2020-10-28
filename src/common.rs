use md5;
use std::time::{SystemTime,UNIX_EPOCH};

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
