

pub fn error(e: &str) -> String {
    println!("!!! {:?}",e);
    return String::from(e);
}

pub fn log(m: &str){
    println!(">>> {:?}",m);
}
