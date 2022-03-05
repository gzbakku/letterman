use tokio::fs::{File,remove_file,create_dir_all};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use std::env::current_dir;

#[allow(dead_code)]
pub async fn read_as_text(path:String) -> Result<String,&'static str>{
    match read_file_raw(path).await{
        Ok(buffer)=>{
            match String::from_utf8(buffer){
                Ok(d)=>{return Ok(d);},
                Err(_)=>{return Err("failed-parse_buffer_to_text")}
            }
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return Err("failed-read_raw_file");
        }
    }
}

#[allow(dead_code)]
pub async fn read_file_raw(path:String) -> Result<Vec<u8>,&'static str>{
    match File::open(path).await{
        Ok(mut reader)=>{
            let mut buffer = Vec::new();
            match reader.read_to_end(&mut buffer).await{
                Ok(_)=>{
                    return Ok(buffer);
                },
                Err(_)=>{
                    return Err("failed-read_file");
                }
            }
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return Err("failed-open_file");
        }
    }
}

#[allow(dead_code)]
pub async fn delete_file(path:String)->Result<(),&'static str>{
    match remove_file(&path).await{
        Ok(_)=>{return Ok(());},
        Err(_)=>{
            return Err("failed-delete_file");
        }
    }
}

#[allow(dead_code)]
pub async fn write_file(path:String,data:Vec<u8>)->Result<(),&'static str>{
    match File::create(path).await{
        Ok(mut reader)=>{
            match reader.write(&data).await{
                Ok(_)=>{
                    return Ok(());
                },
                Err(_)=>{
                    return Err("failed-read_file");
                }
            }
        },
        Err(e)=>{
            println!("!!! {:?}",e);
            return Err("failed-open_file");
        }
    }
}

#[allow(dead_code)]
pub fn cwd()->String{
    match current_dir(){
        Ok(v)=>{
            match v.as_path().to_str(){
                Some(f)=>{
                    f.to_string()
                },
                None=>{
                    String::new()
                }
            }
        },
        Err(_)=>{
            String::new()
        }
    }
}

#[allow(dead_code)]
pub async fn ensure_file_dir(path:String)->Result<(),&'static str>{
    let mut path = path;
    while path.contains("\\"){
        path = path.replace("\\", "/");
    }
    let hold:Vec<&str> = path.split("/").collect();
    let mut collect = String::new();
    for i in 0..hold.len()-1{
        if collect.len() > 0{
            collect = collect + "/" + hold[i];
        } else {
            collect = hold[i].to_string();
        }
    }
    match create_dir_all(collect).await{
        Ok(_)=>{return Ok(());},
        Err(_)=>{return Err("failed-tokio-fs-create_dir_all");}
    }
}

#[allow(dead_code)]
pub async fn ensure_dir(path:String)->Result<(),&'static str>{
    match create_dir_all(path).await{
        Ok(_)=>{return Ok(());},
        Err(_)=>{return Err("failed-tokio-fs-create_dir_all");}
    }
}