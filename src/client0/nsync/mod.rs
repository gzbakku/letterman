mod build;
mod execute;
mod parse;
mod resolve;
pub use build::Email;

#[derive(Debug,Clone)]
pub struct Action {
    pub tag:&'static str,
    pub cate:&'static str,
    pub io:&'static str,
    pub cmd:String
}

pub fn read_key(path:String) -> Result<String,String>{
    match crate::io::read_as_text(path){
        Ok(v)=>{return Ok(v)},
        Err(e)=>{return Err(format!("failed-read_key => {}",e));}
    }
}
