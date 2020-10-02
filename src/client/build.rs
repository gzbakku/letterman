use crate::client::execute::send_mail;

#[derive(Debug,Clone)]
pub struct Email {
    pub name:String,
    pub from:String,
    pub to:String,
    pub cc:String,
    pub bcc:String,
    pub subject:String,
    pub body:String,
    pub attach:Vec<String>,
    pub is_html:bool
}

impl Email{

    pub fn new() -> Email{
        Email{
            name:String::new(),
            from:String::new(),
            to:String::new(),
            cc:String::new(),
            bcc:String::new(),
            subject:String::new(),
            body:String::new(),
            attach:Vec::new(),
            is_html:false
        }
    }
    pub fn name(&mut self,v:String){self.name = v;}
    pub fn to(&mut self,v:String){self.to = v;}
    pub fn from(&mut self,v:String){self.from = v;}
    pub fn cc(&mut self,v:String){
        if self.cc.len() > 0{
            self.cc.push_str(&format!(",{}",v));
        } else {
            self.cc.push_str(&v);
        }
    }
    pub fn bcc(&mut self,v:String){
        if self.bcc.len() > 0{
            self.bcc.push_str(&format!(",{}",v));
        } else {
            self.bcc.push_str(&v);
        }
    }
    pub fn subject(&mut self,v:String){self.subject = v;}
    pub fn body(&mut self,v:String){self.body = v;}
    pub fn attach(&mut self,v:String){self.attach.push(v);}
    pub fn is_html(&mut self){self.is_html = true;}
    pub fn get(self) -> Email{return self;}
    pub fn send(self) -> Result<(),&'static str>{
        match send_mail(self){
            Ok(_)=>{
                return Ok(());
            },
            Err(e)=>{
                return Err("failed-send_mail");
            }
        }
    }

}
