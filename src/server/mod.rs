use tokio_rustls::rustls::{Certificate, PrivateKey};
use tokio_rustls::rustls::ServerConfig as RustlsServerConfig;
use tokio_rustls::TlsAcceptor;
use tokio::net::{TcpListener,TcpStream};
use tokio_rustls::server::TlsStream as TokioRustLsServerTlsStream;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use std::future::Future;
use crate::server::config::{ServerConfig,ServerInfo,QueMessage,Signal};
use tokio::sync::mpsc::{channel};
use tokio::spawn as TokioSpawn;
pub use config::{CheckMail,ProcessMail};
pub use rustque::Config as QueConfig;
use flume::Sender as FlumeSender;
use json::JsonValue;
use std::thread::spawn as ThreadSpawn;

pub mod config;
mod connection;
mod que;
mod jsonify;
mod submit;
mod io;

use io::{load_cert,load_key};

pub const BAD_AUTH:&'static str = "530 Access denied (???a Sendmailism)\r\n";
pub const OK_REPLY:&'static str = "250 2.1.0 Ok\r\n";
pub const QUIT_REPLY:&'static str = "221 goodbye\r\n";
pub const DATA_START:&'static str = "354 Enter mail, end with \".\" on a line by itself\r\n";
pub const BAD_SEQUENCE:&'static str = "503 Bad sequence of commands\r\n";
pub const USER_NOT_FOUND:&'static str = "551 Intended recipient mailbox isn't available on the receiving server\r\n";
pub const DOWN:&'static str = "451 Requested action aborted: local error in processing\r\n";

// pub const UNHANDLED:&'static str = "502 Command not implemented\r\n";
// pub const BAD_SYNTAX:&'static str = "500 Syntax error, command unrecognised\r\n";
// pub const BAD_PARAMS:&'static str = "501 Syntax error in parameters or arguments\r\n";

pub async fn init<F,T,K,V>(
    conf:ServerConfig,
    f:F,
    p:K,
    check_mail_sender:FlumeSender<JsonValue>,
    process_mail_sender:FlumeSender<JsonValue>
)->Result<(),&'static str>
where
    F:Fn(CheckMail,FlumeSender<JsonValue>) -> T + Unpin + Send + 'static + Copy + Sync,
    T:Future<Output = Result<bool,()>> + Send + 'static,
    K:Fn(ProcessMail,FlumeSender<JsonValue>) -> V + Unpin + Send + 'static + Copy + Sync,
    V:Future<Output = Result<(),()>> + Send + 'static,
{

    // println!(">>> initiating server");

    let certs:Vec<Certificate>;
    match load_cert(conf.cert.clone()){
        Ok(v)=>{
            certs = v;
        },
        Err(_)=>{
            return Err("failed-load_cert");
        }
    }

    // println!(">>> load certs inittaited");

    let keys:Vec<PrivateKey>;
    match load_key(conf.key.clone()){
        Ok(v)=>{
            keys = v;
        },
        Err(_)=>{
            return Err("failed-load_key");
        }
    }

    // println!(">>> load keys inittaited");

    if certs.len() == 0 || keys.len() == 0{
        return Err("failed-load_ssl");
    }

    let ports = conf.ports.clone();
    let (que_sender,que_receiver) = channel::<QueMessage>(100);

    let (submit_signal,submit_sleeper) = Signal::new();
    let no_submit_workers = conf.no_of_submitters.clone();
    let que_sender_clone = que_sender.clone();
    let submit_signal_clone = submit_signal.clone();
    if true{
        ThreadSpawn(move || {
            match submit::start(
                que_sender_clone,
                p,
                no_submit_workers,
                process_mail_sender,
                submit_signal_clone
            ){
                Ok(_)=>{},
                Err(_)=>{}
            }
        });
    } else {
        TokioSpawn(async move {
            submit::init(
                que_sender_clone,
                p,
                no_submit_workers,
                process_mail_sender,
                submit_signal_clone
            ).await;
        });
    }
    submit_sleeper.notified().await;
    if !Signal::check(submit_signal).await{
        println!("!!! failed-init-submit");
        return Err("failed-init-submit");
    }

    let conf_clone = conf.clone();
    let (que_signal,que_sleeper) = Signal::new();
    let move_que_signal = que_signal.clone();
    TokioSpawn(async move {
        que::init(
            que_receiver,
            conf_clone.que_path.clone(),
            conf_clone.que_frame_size.clone(),
            conf_clone.que_disk_writers.clone(),
            move_que_signal
        ).await;
    });

    que_sleeper.notified().await;
    if !Signal::check(que_signal).await{
        return Err("failed-init-que");
    }

    let mut hold = Vec::new();
    for port in ports{
        let keys = keys.clone();
        let certs = certs.clone();
        let config_clone = conf.clone();
        let f_clone = f.clone();
        let que_sender_clone = que_sender.clone();
        let check_mail_sender_clone = check_mail_sender.clone();
        hold.push(
            tokio::spawn(async move {
                start_server(
                    port,
                    keys,
                    certs,
                    config_clone,
                    f_clone,
                    que_sender_clone,
                    check_mail_sender_clone
                ).await;
            })
        );
    }

    // println!(">>> server started");

    for i in hold{
        match i.await{
            Ok(_)=>{},
            Err(_)=>{}
        }
    }

    return Err("all server ports closed");

}

async fn start_server<F,T>(
    port:u32,
    mut keys:Vec<PrivateKey>,
    certs:Vec<Certificate>,
    config:ServerConfig,
    check_email:F,
    que_sender:Sender<QueMessage>,
    check_mail_sender:FlumeSender<JsonValue>
)
where
    F:Fn(CheckMail,FlumeSender<JsonValue>) -> T + Unpin + Send + 'static + Copy + Sync,
    T:Future<Output = Result<bool,()>> + Send + 'static,
{

    // println!(">>> server started");

    let rustls_server_config:RustlsServerConfig;
    match RustlsServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth()
    .with_single_cert(certs, keys.remove(0)){
        Ok(v)=>{rustls_server_config = v;},
        Err(_)=>{
            println!("!!! failed-make-RustlsServerConfig");
            return;
        }
    }

    // println!(">>> rustls config created");

    let acceptor = TlsAcceptor::from(Arc::new(rustls_server_config));
    let listener:TcpListener;
    match TcpListener::bind(&format!("0.0.0.0:{}",port)).await{
        Ok(v)=>{listener = v;},
        Err(_e)=>{
            println!("!!! failed-init-TcpListener : {:?}",_e);
            return;
        }
    }

    // println!(">>> server listener started");

    let info:Arc<RwLock<ServerInfo>>;
    match ServerInfo::new(config){
        Ok(v)=>{info = v;},
        Err(_)=>{
            println!("!!! failed-init-ServerInfo");
            return;
        }
    }

    // println!(">>> server info created");

    loop{

        let acceptor = acceptor.clone();
        let info = info.clone();
        let f_clone = check_email.clone();
        let que_sender_clone = que_sender.clone();
        let hold_check_mail_sender = check_mail_sender.clone();

        match listener.accept().await{
            Ok((stream,peer_addr))=>{

                tokio::spawn(async move {

                    let info = info.clone();
                    
                    let secure_stream:TokioRustLsServerTlsStream<TcpStream>;
                    match acceptor.accept(stream).await{
                        Ok(v)=>{secure_stream = v;},
                        Err(_)=>{
                            println!("!!! failed-run-acceptor");
                            return;
                        }
                    }

                    connection::handle(
                        secure_stream,
                        peer_addr,
                        info,
                        f_clone,
                        peer_addr.ip(),
                        que_sender_clone,
                        hold_check_mail_sender
                    ).await;
        
                });

            },
            Err(_)=>{
                println!("!!! failed-run-listener");
            }
        }

    }

}