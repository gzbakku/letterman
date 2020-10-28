use chrono::Utc;

mod sync;
mod parse;
mod nsync;

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

#[derive(Debug,Clone)]
pub struct Email {
    pub dkim_selector:String,
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
    pub attach_base64:Vec<BaseFile>,
    pub is_html:bool,
    pub date:String
}

#[derive(Debug,Clone)]
pub struct BaseFile{
    pub name:String,
    pub base64:String
}

#[allow(dead_code)]
impl Email{

    #[allow(dead_code)]
    pub fn new() -> Email{
        Email{
            dkim_selector:String::from("dkim"),
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
            is_html:false,
            date:Utc::now().to_rfc2822()
        }
    }
    #[allow(dead_code)]
    pub fn dkim_selector(&mut self,v:String){self.dkim_selector = v;}
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
    pub fn attach_base64(&mut self,name:String,base64:String){
        self.attach_base64.push(BaseFile {
            name:name,
            base64:base64
        });
    }
    #[allow(dead_code)]
    pub fn is_html(&mut self){self.is_html = true;}
    #[allow(dead_code)]
    pub fn get(self) -> Email{return self;}
    #[allow(dead_code)]
    pub fn send(self) -> Result<(),&'static str>{
        match sync::send_mail(self){
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                return Err(e);
            }
        }
    }
    #[allow(dead_code)]
    pub async fn send_tokio(self) -> Result<(),&'static str>{
        match nsync::send_mail(self).await{
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                return Err(e);
            }
        }
    }

}
