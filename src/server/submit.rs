use std::future::Future;
use crate::server::config::{QueMessage,ProcessMail,SignalData,QueGetMessage,QueRemoveMessage,QueResetMessage,Signal,SignalDataHolder};
use tokio::sync::mpsc::Sender;
use tokio::spawn as TokioSpawn;
use tokio::time::{sleep, Duration};
use json::parse as JsonParse;
use flume::Sender as FlumeSender;
use json::JsonValue;
use tokio::runtime::Builder as TokioRuntimeBuilder;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn start<K,V>(
    sender:Sender<QueMessage>,
    f:K,
    no_of_workers:u16,
    process_mail_sender:FlumeSender<JsonValue>,
    signal:Arc<Mutex<Signal>>
)->Result<(),&'static str>
where
    K:Fn(ProcessMail,FlumeSender<JsonValue>) -> V + Unpin + Send + 'static + Copy + Sync,
    V:Future<Output = Result<(),()>> + Send + 'static,
{

    let build = TokioRuntimeBuilder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    // .thread_name("my-custom-name")
    // .thread_stack_size(3 * 1024 * 1024)
    .build();

    match build{
        Ok(runtime)=>{
            runtime.block_on(async {
                init(
                    sender,
                    f,
                    no_of_workers,
                    process_mail_sender,
                    signal
                ).await;
            });
            println!("all submit started");
            return Ok(());
        },
        Err(_)=>{
            return Err("failed-build-runtime");
        }
    }

    // for _ in 0..no_of_workers{
    //     let sender_hold = sender.clone();
    //     let f_hold = f.clone();
    //     let process_mail_sender_clone = process_mail_sender.clone();
    //     TokioSpawn(async move{
    //         init_worker(
    //             sender_hold,
    //             f_hold,
    //             process_mail_sender_clone
    //         ).await;
    //     });
    // }

}

pub async fn init<K,V>(
    sender:Sender<QueMessage>,
    f:K,
    no_of_workers:u16,
    process_mail_sender:FlumeSender<JsonValue>,
    signal:Arc<Mutex<Signal>>
)
where
    K:Fn(ProcessMail,FlumeSender<JsonValue>) -> V + Unpin + Send + 'static + Copy + Sync,
    V:Future<Output = Result<(),()>> + Send + 'static,
{

    // println!("submit init started");

    let mut collect = Vec::new();
    for _ in 0..no_of_workers{
        let sender_hold = sender.clone();
        let f_hold = f.clone();
        let process_mail_sender_clone = process_mail_sender.clone();
        collect.push(
            TokioSpawn(async move {
                init_worker(
                    sender_hold,
                    f_hold,
                    process_mail_sender_clone
                ).await;
            })
        );
    }

    Signal::ok(signal).await;

    for i in collect{
        match i.await{
            Ok(_)=>{},
            Err(_)=>{}
        }
    }

}

async fn init_worker<K,V>(sender:Sender<QueMessage>,f:K,process_mail_sender:FlumeSender<JsonValue>)
where
    K:Fn(ProcessMail,FlumeSender<JsonValue>) -> V + Unpin + Send + 'static + Copy + Sync,
    V:Future<Output = Result<(),()>> + Send + 'static,
{

    loop {

        let hold_process_mail_sender = process_mail_sender.clone();
        let (signal,sleeper) = SignalData::new();

        match sender.send(QueMessage::Get(
            QueGetMessage{
                signal:signal.clone()
            }
        )).await{
            Ok(_)=>{

                // println!("sent");

                sleeper.notified().await;

                if SignalData::check(&signal).await{

                    let data = SignalData::data(signal).await;
                    let data_index = data.index.clone();
                    match parse_object(data){
                        Ok((data,files))=>{

                            match f(ProcessMail{
                                email:data,
                                files:files
                            },hold_process_mail_sender).await{
                                Ok(_)=>{
                                    let (signal,sleeper) = Signal::new();
                                    match sender.send(QueMessage::Remove(
                                        QueRemoveMessage{
                                            signal:signal,
                                            index:data_index
                                        }
                                    )).await{Ok(_)=>{
                                        sleeper.notified().await;
                                    },Err(_)=>{}}
                                },
                                Err(_)=>{
                                    let (signal,sleeper) = Signal::new();
                                    match sender.send(QueMessage::Reset(
                                        QueResetMessage{
                                            signal:signal,
                                            index:data_index
                                        }
                                    )).await{Ok(_)=>{
                                        sleeper.notified().await;
                                    },Err(_)=>{}}
                                    sleep(Duration::from_millis(1000)).await;
                                }
                            }
                        },
                        Err(_e)=>{
                            println!("!!! failed-parse_object : {:?}",_e);
                            let (signal,_) = Signal::new();
                            match sender.send(QueMessage::Remove(
                                QueRemoveMessage{
                                    signal:signal,
                                    index:data_index
                                }
                            )).await{Ok(_)=>{},Err(_)=>{}}
                            // sleep(Duration::from_millis(1000)).await;
                        }
                    }

                } else {
                    sleep(Duration::from_millis(1000)).await;
                }

            },
            Err(_)=>{
                sleep(Duration::from_millis(1000)).await;
            }
        }

    }

}

fn parse_object(data:SignalDataHolder)->Result<(JsonValue,Vec<String>),&'static str>{

    let as_string:String;
    match String::from_utf8(data.data){
        Ok(v)=>{
            as_string = v;
        },
        Err(_)=>{
            return Err("failed-parse_mail");
        }
    }

    let as_json:JsonValue;
    match JsonParse(&as_string){
        Ok(v)=>{
            as_json = v;
        },
        Err(_)=>{
            return Err("failed-parse_mail");
        }
    }

    //build
    // object!{
    //     "id":id_hash.clone(),
    //     "from":email.from,
    //     "to":email.to,
    //     "headers":{},
    //     "body":[],
    //     "attachments":[]
    // }

    //body
    // object!{
    //     "type":"string",
    //     "value":v
    // }

    //attachments
    // object!{
    //     "type":"file",
    //     "value":v
    // }

    // object!{
    //     name:name,
    //     hash:name_hash,
    //     file_name:file_name.clone(),
    //     features:{},
    //     content_features:{}
    // }

    if !as_json["id"].is_string(){return Err("no-id");}
    if !as_json["from"].is_string(){return Err("no-from");}
    if !as_json["to"].is_string(){return Err("no-to");}
    if !as_json["headers"].is_object(){return Err("no-headers");}
    if !as_json["body"].is_array(){return Err("no-body");}
    if !as_json["attachments"].is_array(){return Err("no-attachments");}

    let mut file_names:Vec<String> = vec![];
    for item in as_json["body"].members(){
        if !item["type"].is_string(){return Err("no-type");}
        if !item.has_key("value"){return Err("no-value");}
        match item["type"].as_str(){
            Some(v)=>{
                if v == "file"{
                    match item["value"]["file_name"].as_str(){
                        Some(file_name)=>{
                            file_names.push(file_name.to_string());
                        },
                        None=>{return Err("not_found-file_name");}
                    }
                }
            },
            None=>{return Err("not_found-type");}
        }
    }

    for item in as_json["attachments"].members(){
        if !item["type"].is_string(){return Err("no-type");}
        if !item["value"].is_object(){return Err("no-value");}
        match item["type"].as_str(){
            Some(v)=>{
                if v == "file"{
                    match item["value"]["file_name"].as_str(){
                        Some(file_name)=>{
                            file_names.push(file_name.to_string());
                        },
                        None=>{return Err("not_found-file_name");}
                    }
                } else {
                    return Err("invalid_file_type");
                }
            },
            None=>{return Err("not_found-type");}
        }
    }

    return Ok((as_json,file_names));

}