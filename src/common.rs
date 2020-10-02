

pub fn error(e: &'static str) -> String {
    println!("!!! {:?}",e);
    return String::from(e);
}

pub fn log(m: &str){
    println!(">>> {:?}",m);
}
