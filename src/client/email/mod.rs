use chrono::Utc;
// use openssl::pkey::{PKey,Private};
use crate::client::Connection;

mod parse;

#[derive(Debug,Clone)]
pub struct Action {
    pub tag:&'static str,
    pub cate:&'static str,
    pub io:&'static str,
    pub cmd:String
}

#[derive(Debug,Clone)]
pub struct Email {
    pub tracking_id:String,
    pub unique_id:String,
    pub private_key:String,
    pub server_name:String,
    pub name:String,
    pub from:String,
    pub to:String,
    pub cc:String,
    pub bcc:String,
    pub subject:String,
    pub body:String,
    pub attach:Vec<String>,
    pub attach_base64:Vec<(String,String,String)>,//file_name,base64,file_mime
    pub html:String,
    pub date:String
}

#[allow(dead_code)]
impl Email{

    #[allow(dead_code)]
    pub fn new() -> Email{
        Email{
            tracking_id:String::new(),
            unique_id:String::new(),
            private_key:String::new(),
            server_name:String::new(),
            name:String::new(),
            from:String::new(),
            to:String::new(),
            cc:String::new(),
            bcc:String::new(),
            subject:String::new(),
            body:String::new(),
            attach:Vec::new(),
            attach_base64:Vec::new(),
            html:String::new(),
            date:Utc::now().to_rfc2822()
        }
    }
    #[allow(dead_code)]
    pub fn tracking_id(&mut self,v:String){self.tracking_id = v;}
    #[allow(dead_code)]
    pub fn unique_id(&mut self,v:String){self.unique_id = v;}
    #[allow(dead_code)]
    pub fn private_key(&mut self,v:String){self.private_key = v;}
    #[allow(dead_code)]
    pub fn server_name(&mut self,v:String){self.server_name = v;}
    #[allow(dead_code)]
    pub fn name(&mut self,v:String){self.name = v;}
    #[allow(dead_code)]
    pub fn to(&mut self,v:String){self.to = v;}
    #[allow(dead_code)]
    pub fn from(&mut self,v:String){self.from = v;}
    #[allow(dead_code)]
    pub fn cc(&mut self,v:String){
        if self.cc.len() > 0{
            self.cc.push_str(&format!(",{}",v));
        } else {
            self.cc.push_str(&v);
        }
    }
    #[allow(dead_code)]
    pub fn bcc(&mut self,v:String){
        if self.bcc.len() > 0{
            self.bcc.push_str(&format!(",{}",v));
        } else {
            self.bcc.push_str(&v);
        }
    }
    #[allow(dead_code)]
    pub fn subject(&mut self,v:String){self.subject = v;}
    #[allow(dead_code)]
    pub fn body(&mut self,v:String){self.body = v;}
    #[allow(dead_code)]
    pub fn attach(&mut self,v:String){self.attach.push(v);}
    #[allow(dead_code)]
    pub fn attach_base64(&mut self,name:String,base64:String,mime:String){
        self.attach_base64.push((name,base64,mime));
    }
    #[allow(dead_code)]
    pub fn html(&mut self,body:String){self.html = body;}
    #[allow(dead_code)]
    pub fn get(self) -> Email{return self;}
    pub async fn parse(self,conn:&Connection)->Result<(Vec<String>,String,u64),&'static str>{
        match parse::init(self,conn).await{
            Ok(v)=>{
                return Ok(v);
            },
            Err(_e)=>{
                return Err(_e);
            }
        }
    }
}