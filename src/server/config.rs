use tokio_spf_validator::SpfConfig;
use regex::Regex;
use tokio::sync::RwLock;
use std::sync::Arc;
use letterman_email_body_parser::Config as EmailBodyParserConfig;
use tokio_spf_validator::SpfQueryResult;
use tokio::sync::{Mutex,Notify};
use json::JsonValue;
use crate::io::{ensure_file_dir,ensure_dir};

#[derive(Debug,Clone)]
pub struct CheckMail{
    pub from:String,
    pub to:String
}

#[derive(Debug,Clone)]
pub struct ProcessMail{
    pub email:JsonValue,
    pub files:Vec<String>
}

#[derive(Clone,Debug)]
pub struct ServerConfig
{
    pub email_body_parser_config:EmailBodyParserConfig,
    pub ports:Vec<u32>,
    pub server_name:String,
    pub cert:String,
    pub key:String,
    pub size:u128,
    pub validate_rdns:bool,
    pub validate_spf:bool,
    pub validate_dkim:bool,
    pub que_path:String,
    pub que_frame_size:u64,
    pub que_disk_writers:u64,
    pub attachment_storage_dir:String,
    pub no_of_submitters:u16
}

impl ServerConfig{
    pub async fn new(
        ports:Vec<u32>,
        server_name:String,
        cert_path:String,
        key_path:String,
        max_email_size:u128,
        que_path:String,
        que_frame_size:u64,
        que_disk_writers:u64,
        attachment_storage_dir:String,
        no_of_submitters:u16,
        validate_rdns:bool,
        validate_spf:bool,
        validate_dkim:bool,
    )->Result<ServerConfig,&'static str>{

        let email_body_parser_config:EmailBodyParserConfig;
        match EmailBodyParserConfig::new(){
            Ok(v)=>{email_body_parser_config = v;},
            Err(_)=>{return Err("failed-init-EmailBodyParserConfig");}
        }

        match ensure_file_dir(que_path.clone()).await{
            Ok(_)=>{},
            Err(_)=>{return Err("failed-ensure-path-");}
        }
        match ensure_dir(attachment_storage_dir.clone()).await{
            Ok(_)=>{},
            Err(_)=>{return Err("failed-ensure-path-");}
        }

        return Ok(ServerConfig{
            email_body_parser_config:email_body_parser_config,
            ports:ports,
            server_name:server_name,
            cert:cert_path,
            key:key_path,
            size:max_email_size,
            validate_rdns:validate_rdns,
            validate_spf:validate_spf,
            validate_dkim:validate_dkim,
            que_path:que_path,
            que_frame_size:que_frame_size,
            que_disk_writers:que_disk_writers,
            attachment_storage_dir:attachment_storage_dir,
            no_of_submitters:no_of_submitters
        });

    }
}

#[derive(Clone)]
pub struct ServerInfo{
    pub email_body_parser_config:EmailBodyParserConfig,
    pub name:String,
    pub size:u128,
    pub message_end:Vec<u8>,
    pub data_end:Vec<u8>,
    pub spf_config:SpfConfig,
    pub regex:ServerRegex,
    pub validate_rdns:bool,
    pub validate_spf:bool,
    pub validate_dkim:bool,
    pub attachment_storage_dir:String
}

impl ServerInfo{
    pub fn new(config:ServerConfig)->Result<Arc<RwLock<ServerInfo>>,&'static str>{

        let spf_config:SpfConfig;
        match SpfConfig::new(){
            Ok(v)=>{spf_config = v;},
            Err(_)=>{return Err("failed-init-spfConfig");}
        }

        let server_regex:ServerRegex;
        match ServerRegex::new(){
            Ok(v)=>{server_regex = v;},
            Err(_)=>{return Err("failed-init-server_regex");}
        }

        return Ok(Arc::new(RwLock::new(ServerInfo{
            email_body_parser_config:config.email_body_parser_config,
            name:config.server_name.clone(),
            size:config.size,
            message_end:"\r\n".as_bytes().to_vec(),
            data_end:"\r\n.\r\n".as_bytes().to_vec(),
            spf_config:spf_config,
            regex:server_regex,
            validate_rdns:config.validate_rdns,
            validate_spf:config.validate_spf,
            validate_dkim:config.validate_dkim,
            attachment_storage_dir:config.attachment_storage_dir
        })));

    }
}

#[derive(Clone,Debug)]
pub struct ServerRegex{
    pub ehlo:Regex,
    pub email:Regex
}

impl ServerRegex{
    pub fn new()->Result<ServerRegex,&'static str>{

        let ehlo_regex:Regex;
        match Regex::new(r"EHLO\s+([\w.]+)\s*\r\n"){
            Ok(v)=>{ehlo_regex = v;},
            Err(_)=>{return Err("failed-init-ehlo_regex");}
        }

        let email_regex:Regex;
        match Regex::new(r#"([\w\d_=+/*!@#$%^&*()-|]+)@([\w\d.]+)"#){
            Ok(v)=>{email_regex = v;},
            Err(_)=>{
                return Err("failed-regex-from_regex");
            }
        }

        return Ok(ServerRegex{
            ehlo:ehlo_regex,
            email:email_regex
        });

    }
}

#[derive(Debug)]
pub struct QueAddMessage{
    pub signal:Arc<Mutex<Signal>>,
    pub body:Vec<u8>
}

#[derive(Debug)]
pub struct QueGetMessage{
    pub signal:Arc<Mutex<SignalData>>
}

#[derive(Debug)]
pub struct QueRemoveMessage{
    pub signal:Arc<Mutex<Signal>>,
    pub index:u64
}

#[derive(Debug)]
pub struct QueResetMessage{
    pub signal:Arc<Mutex<Signal>>,
    pub index:u64
}

#[derive(Debug)]
pub enum QueMessage{
    Add(QueAddMessage),
    Get(QueGetMessage),
    Remove(QueRemoveMessage),
    Reset(QueResetMessage)
}

#[derive(Clone,Debug)]
pub struct Email{
    pub spf:SpfQueryResult,
    pub status:u8,
    pub from:String,
    pub to:String
}

impl Email{
    pub fn new()->Email{
        Email{
            spf:SpfQueryResult::Pass,
            status:0,
            from:String::new(),
            to:String::new()
        }
    }
    pub fn reset(&mut self){
        self.spf = SpfQueryResult::Pass;
        self.status = 0;
        self.from.clear();
        self.to.clear();
    }
    pub fn flush(&mut self)->Email{
        let hold = self.clone();
        self.reset();
        return hold;
    }
}

#[derive(Debug)]
pub struct Signal{
    pub result:bool,
    pub waker:Arc<Notify>
}

impl Signal{
    pub fn new()->(Arc<Mutex<Signal>>,Arc<Notify>){
        let sleeper = Arc::new(Notify::new());
        (
            Arc::new(
                Mutex::new(
                    Signal{
                        result:false,
                        waker:sleeper.clone()
                    }
                )
            ),
            sleeper
        )
    }
    pub async fn ok(hold:Arc<Mutex<Signal>>){
        let mut lock = hold.lock().await;
        lock.result = true;
        lock.waker.notify_one();
    }
    pub async fn wait(hold:Arc<Notify>){
        hold.notified().await;
    }
    pub async fn error(hold:Arc<Mutex<Signal>>){
        let lock = hold.lock().await;
        lock.waker.notify_one();
    }
    pub async fn check(hold:Arc<Mutex<Signal>>)->bool{
        let lock = hold.lock().await;
        return lock.result;
    }
}

#[derive(Clone,Debug,Default)]
pub struct SignalDataHolder{
    pub index:u64,
    pub data:Vec<u8>
}

impl SignalDataHolder{
    pub fn new()->SignalDataHolder{
        SignalDataHolder::default()
    }
    pub fn update(&mut self,index:u64,data:Vec<u8>){
        self.index = index;
        self.data = data;
    }
}

#[derive(Debug)]
pub struct SignalData{
    pub result:bool,
    pub data:SignalDataHolder,
    pub waker:Arc<Notify>
}

impl SignalData{
    pub fn new()->(Arc<Mutex<SignalData>>,Arc<Notify>){
        let sleeper = Arc::new(Notify::new());
        (
            Arc::new(
                Mutex::new(
                    SignalData{
                        result:false,
                        data:SignalDataHolder::new(),
                        waker:sleeper.clone()
                    }
                )
            ),
            sleeper
        )
    }
    pub async fn check(hold:&Arc<Mutex<SignalData>>)->bool{
        let lock = hold.lock().await;
        return lock.result;
    }
    pub async fn data(hold:Arc<Mutex<SignalData>>)->SignalDataHolder{
        let lock = hold.lock().await;
        return lock.data.clone();
    }
    pub async fn update(hold:Arc<Mutex<SignalData>>,index:u64,data:Vec<u8>){
        let mut lock = hold.lock().await;
        lock.result = true;
        lock.data.update(index,data);
        lock.waker.notify_one();
    }
    pub async fn error(hold:Arc<Mutex<SignalData>>){
        let lock = hold.lock().await;
        lock.waker.notify_one();
    }
}