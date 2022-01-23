use crate::server::config::{QueMessage,Signal,SignalData};
use tokio::sync::mpsc::Receiver;
use rustque::{Que,Config};
use std::sync::Arc;
use tokio::sync::Mutex;
// use crate::common::hash as md5_hash;

fn error(e:&'static str){
    println!("!!! {:?}",e);
    // panic!("que error");
}

pub async fn init(
    receiver:Receiver<QueMessage>,
    que_files:Vec<String>,
    que_min_size:u64,
    que_expand_size:u64,
    que_disk_writers:u8,
    signal:Arc<Mutex<Signal>>
){

    let mut receiver = receiver;
    let mut que:Que;
    match Que::new(Config::new(
        que_files,
        que_min_size,
        que_expand_size,
        que_disk_writers
    )).await{
        Ok(v)=>{que = v;},
        Err(_e)=>{
            Signal::error(signal).await;
            return error("failed-init-que");
        }
    }

    Signal::ok(signal).await;

    // println!("{:?}",que);

    // println!(">>> que initiated");

    loop{

        match receiver.recv().await{
            Some(message)=>{
                match message{
                    QueMessage::Add(add)=>{
                        match que.add(add.body).await{
                            Ok(que_response)=>{
                                if que_response.check().await{
                                    Signal::ok(add.signal).await;
                                } else {
                                    Signal::error(add.signal).await;
                                }
                            },
                            Err(_)=>{
                                Signal::error(add.signal).await;
                            }
                        }
                    },
                    QueMessage::Get(get)=>{
                        match que.next().await{
                            Ok(que_response)=>{
                                if !que_response.check().await{
                                    SignalData::error(get.signal).await;
                                } else {
                                    match que_response.data().await{
                                        Some((value,pointer))=>{
                                            SignalData::update(get.signal,pointer,value).await;
                                        },
                                        None=>{
                                            SignalData::error(get.signal).await;
                                        }
                                    }
                                }
                            },
                            Err(_)=>{
                                SignalData::error(get.signal).await;
                            }
                        }
                    },QueMessage::Remove(remove)=>{
                        match que.remove(remove.pointer).await{
                            Ok(que_response)=>{
                                if que_response.check().await{
                                    Signal::ok(remove.signal).await;
                                } else {
                                    Signal::error(remove.signal).await;
                                }
                            },
                            Err(_)=>{
                                Signal::error(remove.signal).await;
                            }
                        }
                    },
                    QueMessage::Reset(reset)=>{
                        match que.reset(reset.pointer).await{
                            Ok(que_response)=>{
                                if que_response.check().await{
                                    Signal::ok(reset.signal).await;
                                } else {
                                    Signal::error(reset.signal).await;
                                }
                            },
                            Err(_)=>{
                                Signal::error(reset.signal).await;
                            }
                        }
                    }
                }
            },
            None=>{
                error("receive_failed");
                break;
            }
        }

    }

}