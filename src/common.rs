
#[allow(dead_code)]
pub fn error(e: &'static str) -> String {
    println!("!!! {:?}",e);
    return String::from(e);
}

#[allow(dead_code)]
pub fn log(m: &str){
    println!(">>> {:?}",m);
}
