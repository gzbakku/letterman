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
    que_path:String,
    que_frame_size:u64,
    que_disk_writers:u64,
    signal:Arc<Mutex<Signal>>
){

    let mut receiver = receiver;
    let mut que:Que;
    match Que::new(Config::new(
        que_path,
        que_frame_size,
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
                            Ok(_)=>{
                                Signal::ok(add.signal).await;
                            },
                            Err(_)=>{
                                Signal::error(add.signal).await;
                            }
                        }
                    },
                    QueMessage::Get(get)=>{
                        match que.get().await{
                            Ok(v)=>{
                                SignalData::update(get.signal,v.1,v.0).await;
                            },
                            Err(_)=>{
                                SignalData::error(get.signal).await;
                            }
                        }
                    },QueMessage::Remove(get)=>{
                        match que.remove(get.index).await{
                            Ok(_)=>{
                                Signal::ok(get.signal).await;
                            },
                            Err(_)=>{
                                Signal::error(get.signal).await;
                            }
                        }
                    },
                    QueMessage::Reset(get)=>{
                        match que.reset(get.index).await{
                            Ok(_)=>{
                                Signal::ok(get.signal).await;
                            },
                            Err(_)=>{
                                Signal::error(get.signal).await;
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