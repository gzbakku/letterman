use crate::client::email::Email;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::client::io::read_file;
// use crate::client::io::write_file;
use crate::client::{Connection};
use openssl::sha::Sha256;
use openssl::sign::Signer;
use openssl::pkey::{PKey,Private};
use openssl::hash::MessageDigest;
use quoted_printable::encode_to_str as QPEncoder;
// use base64::encode as Base64Encoder;

pub async fn init(email:Email,conn:&Connection)->Result<(Vec<String>,String,u64),&'static str>{

    if email.receivers.len() == 0{
        return Err("not_found-receivers-parse_email");
    }

    let mut commands = vec![];
    let mut headers = String::new();
    let mut dkim_headers = String::new();
    let mut dkim = String::new();

    if email.name.len() > 0{
        // commands.push(format!("MAIL FROM:{} <{}>\r\n",email.name,email.from));
        commands.push(format!("MAIL FROM:<{}>\r\n",email.from));
        headers.push_str(&format!("From:{} <{}>\r\n",email.name,email.from));
        dkim_headers.push_str(&format!("from:{} <{}>\r\n",email.name,email.from));
    } else {
        commands.push(format!("MAIL FROM: <{}>\r\n",email.from));
        headers.push_str(&format!("FROM: <{}>\r\n",email.from));
        dkim_headers.push_str(&format!("from:<{}>\r\n",email.from));
    }
    for i in email.receivers.iter(){
        commands.push(format!("RCPT TO:<{}>\r\n",i));
    }
    // commands.push(format!("RCPT TO:<{}>\r\n",email.to));
    commands.push(format!("DATA\r\n"));

    
    if email.unique_id.len() == 0{
        let uid = format!("{}.{}@{}",unique_id(32),&conn.server_name,&conn.domain);
        headers.push_str(&format!("Message-ID: <{}>\r\n",uid));
        dkim_headers.push_str(&format!("message-id:<{}>\r\n",uid));
    } else {
        headers.push_str(&format!("Message-ID: <{}>\r\n",email.unique_id));
        dkim_headers.push_str(&format!("message-id:<{}>\r\n",email.unique_id));
    }

    let mut to_value = String::new();
    for i in email.to{
        if to_value.len() == 0{
            to_value += &format!("{}",i);
        } else {
            to_value += &format!(", {}",i);
        }
    }

    headers.push_str(&format!("Date: {}\r\n",email.date));
    headers.push_str(&format!("Subject: {}\r\n",email.subject));
    headers.push_str(&format!("To: {}\r\n",to_value));
    headers.push_str(&format!("DKIM-Signature: ===dkim===\r\n"));

    dkim_headers.push_str(&format!("date:{}\r\n",email.date));
    dkim_headers.push_str(&format!("subject:{}\r\n",email.subject));
    dkim_headers.push_str(&format!("to:{}\r\n",to_value));

    if email.body.len() == 0 && email.html.len() == 0{
        return Err("invalid_email_body");
    }

    let mut with_files = false;
    let mut only_html = false;
    let mut alternatives = false;
    if email.body.len() > 0 && email.html.len() > 0{alternatives = true;}
    if email.attach.len() > 0 || email.attach_base64.len() > 0{with_files = true;}
    if email.html.len() > 0 && email.body.len() == 0 && !with_files{only_html = true;}

    // println!("\nwith_files : {:?}",with_files);
    // println!("alternatives : {:?}\n",alternatives);

    if with_files || alternatives{
        headers.push_str(&format!("MIME-Version: 1.0\r\n"));
    }
    if (with_files && alternatives) || with_files{
        headers.push_str(&format!(r#"Content-Type: multipart/mixed; boundary="000000000000dbc95d05d2847226""#));
        headers.push_str(&"\r\n");
    } else if alternatives{
        headers.push_str(&format!(r#"Content-Type: multipart/alternative; boundary="000000000000dbc95d05d2847225""#));
        headers.push_str(&"\r\n");
    } else if only_html{
        headers.push_str(&format!(r#"Content-Type: text/plain; charset="UTF-8""#));
        headers.push_str(&"\r\n");
    }

    let mut body = String::from("");
    if (with_files && alternatives) || with_files{
        body.push_str(&format!("--000000000000dbc95d05d2847226\r\n"));
    } else if alternatives{
        body.push_str(&format!("--000000000000dbc95d05d2847225\r\n"));
    }

    if alternatives && with_files{
        body.push_str(&format!(r#"Content-Type: multipart/alternative; boundary="000000000000dbc95d05d2847225""#));
        body.push_str(&"\r\n");body.push_str(&"\r\n");
        body.push_str(&format!("--000000000000dbc95d05d2847225\r\n"));
    }
    if alternatives{
        body.push_str(&format!(r#"Content-Type: text/plain; charset="UTF-8""#));
        body.push_str(&"\r\n");body.push_str(&"\r\n");
        body.push_str(&format!("{}\r\n",clean_text(&email.body)));
        body.push_str(&format!("--000000000000dbc95d05d2847225\r\n"));
        body.push_str(&format!(r#"Content-Type: text/html; charset="UTF-8""#));
        // body.push_str(&format!(r#"Content-Type: text/html; charset="US-ASCII""#));
        body.push_str(&format!("\r\nContent-Transfer-Encoding: quoted-printable"));
        // body.push_str(&format!("\r\nContent-Transfer-Encoding: base64"));
        // println!("where");
        // body.push_str(&"\r\n");
        // body.push_str(&format!("\r\nContent-Transfer-Encoding: 8BIT"));
        body.push_str(&"\r\n");body.push_str(&"\r\n");
        body.push_str(&format!("{}\r\n",clean_html(&email.html)));
        if with_files{
            body.push_str(&format!("--000000000000dbc95d05d2847225--\r\n"));
        }
    }
    if !alternatives && with_files{
        if email.body.len() > 0{
            body.push_str(&format!(r#"Content-Type: text/plain; charset="UTF-8""#));
            body.push_str(&"\r\n");body.push_str(&"\r\n");
            body.push_str(&format!("{}\r\n",clean_text(&email.body)));
        }
        if email.html.len() > 0{
            body.push_str(&format!(r#"Content-Type: text/plain; charset="UTF-8""#));
            body.push_str(&"\r\n");body.push_str(&"\r\n");
            body.push_str(&format!("{}\r\n",clean_html(&email.html)));
        }
    }
    if with_files{
        if email.attach.len() > 0{
            match parse_files(email.attach).await{
                Ok(v)=>{
                    body.push_str(&v);
                },
                Err(_)=>{
                    return Err("failed-parse_files");
                }
            }
        }
        if email.attach_base64.len() > 0{
            body.push_str(&parse_base64_attachments(email.attach_base64));
        }
    }
    
    if (with_files && alternatives) || with_files{
        body.push_str(&format!("--000000000000dbc95d05d2847226--\r\n"));
    } else if alternatives{
        body.push_str(&format!("--000000000000dbc95d05d2847225--\r\n"));
    }

    if !with_files && !alternatives && !only_html{
        body.push_str(&format!("{}\r\n",clean_text(&email.body)));
    }
    if !with_files && !alternatives && only_html{
        body.push_str(&format!("{}\r\n",clean_html(&email.html)));
    }

    // println!("\n-----------\n{:?}\n-----------\n",body);

    body = clean_bytes_from_string(body);

    dkim.push_str(&"v=1;");
    dkim.push_str(&" a=rsa-sha256;");
    dkim.push_str(&" q=dns/txt;");
    dkim.push_str(&" c=relaxed/relaxed;");
    dkim.push_str(&format!(" d={};",&conn.sender_domain));
    dkim.push_str(&format!(" s={};",&conn.dkim_selector));
    dkim.push_str(&" h=from:message-id:date:subject:to;");
    dkim.push_str(&format!(" bh={};",&hash_sha256(body.clone())));
    dkim.push_str(&" b=");

    let dkim_signature_string = dkim_headers + "dkim-signature:" + &dkim;

    // println!("\n-------\n\n{}\n\n-------\n",headers);
    // println!("\n-------\n\n{:?}\n\n-------\n",body);
    // println!("\n-------\n\n dkim_signature_string : {:?}\n\n-------\n",dkim_signature_string);

    match sign_here(&conn.private_key,dkim_signature_string.into_bytes()){
        Ok(dkim_signature)=>{
            // println!("\n-------\n\n dkim_signature : {:?}\n\n-------\n",dkim_signature);
            dkim = dkim.replace("b=",&format!("b={}",dkim_signature));
        },
        Err(_)=>{
            return Err("failed-sign");
        }
    }

    headers = headers.replace("===dkim===",&dkim);
    commands.push(format!("{}\r\n{}\r\n.\r\n",headers,body));

    // println!("{:?}",commands);

    // match crate::io::write_file(
    //     "D:\\workstation\\expo\\rust\\letterman\\letterman_email_body_parser\\ge_html_wwe.txt".to_string(),
    //     commands[4].clone().as_bytes().to_vec()
    // ).await{
    //     Ok(_)=>{
    //         println!(">>>>>>>>>>>>>>>>> write complete");
    //     },
    //     Err(_e)=>{
    //         println!("!!!!! failed-make sample file : {:?}",_e);
    //     }
    // }
    // return Err("no_error");

    // println!("\n-------\n{}\n-------\n",headers);
    // write_file(commands[3].clone()).await;
    // println!("\n-------\n\n{}\n\n-------\n",commands[3]);

    let mut size:u64 = 0;
    size += commands[0].len() as u64;
    size += commands[1].len() as u64;
    size += commands[2].len() as u64;
    size += commands[3].len() as u64;

    // println!("{}",commands[3]);

    return Ok((commands,email.tracking_id,size));

}

fn clean_bytes_from_string(body:String)->String{

    if true{
        return body;
    }

    let as_bytes = body.as_bytes().to_vec();
    let chars:Vec<char> = body.chars().collect();

    println!("len1 : {}",chars.len());
    println!("len2 : {}",as_bytes.len());

    if false{
        let mut index = 0;
        for c in chars{
            if as_bytes[index] < 32 || as_bytes[index] > 126{
                print!("{} {:?} {} ,\n",index,c,as_bytes[index]);
            }
            index += 1;
        }
    }

    match String::from_utf8(as_bytes){
        Ok(v)=>{
            println!("{}",v);
            return v;
        },
        Err(_)=>{
            return String::new();
        }
    }

    // return body;

}

fn clean_text(text:&String)->String{
    let base = text.to_string();
    let clean_1 = base.trim_start().to_string();
    let mut build = clean_1.trim_end().to_string();
    build = replace_text(build, "\r\n", "+999++9+999+9+9+999+9");
    build = replace_text(build, "\n", "+999++9+999+9+9+999+9");
    let pool:Vec<&str> = build.split("+999++9+999+9+9+999+9").collect();
    let mut rebuild = String::new();
    for mut i in pool{
        i = i.trim_start();
        i = i.trim_end();
        if i.len() > 0{
            // let h = ;
            let k = &build_line(i.to_string());
            i = k;
            // println!("{:?}",i);
            if rebuild.len() > 0{
                rebuild.push_str(&format!("\r\n{}",i));
            } else {
                rebuild.push_str(&format!("{}",i));
            }
        }
    }
    rebuild = replace_text(rebuild, "\r\n.\r\n", ".\r\n");
    return rebuild;
}

fn clean_html(text:&String)->String{
    let mut base = clean_text(text);
    base = replace_text(base, "\r\n", "");
    base = build_line(base);
    base = QPEncoder(base);
    return base;
}

fn replace_text(mut text:String,find:&'static str,replace:&'static str)->String{
    loop{
        if !text.contains(find){
            break;
        }
        text = text.replace(find,replace);
    }
    return text;
}

fn build_line(mut m:String)->String{
    let mut build = String::new();
    loop{
        if m.len() > 1000{
            let (first, last) = m.split_at(990);
            build.push_str(&format!("{}\r\n",first));
            m = last.to_string();
        } else {
            build.push_str(&format!("{}",m));
            break;
        }
    }
    return build;
}

async fn parse_files(files:Vec<String>)->Result<String,&'static str>{
    let mut build = String::new();
    for path in files.iter(){
        match read_file(path.to_string()).await{
            Ok(file)=>{
                build.push_str(&format!("--000000000000dbc95d05d2847226\r\n"));
                build.push_str(&format!(r#"Content-Type: {}; charset="UTF-8"; name="{}""#,&file.1,&file.0));
                build.push_str(&"\r\n");
                build.push_str(&format!(r#"Content-Disposition: attachment; filename="{}""#,&file.0));
                build.push_str(&"\r\n");
                build.push_str(&format!("Content-Transfer-Encoding: base64\r\n\r\n"));
                build.push_str(&format!("{}\r\n",build_line(file.2)));
            },
            Err(_)=>{
                return Err("failed-read_file");
            }
        }
    }
    return Ok(build);
}

fn parse_base64_attachments(attachments:Vec<(String,String,String)>)->String{
    let mut build = String::new();
    for attachment in attachments.iter(){
        build.push_str(&format!("--000000000000dbc95d05d2847226\r\n"));
        build.push_str(&format!(r#"Content-Type: {}; charset="UTF-8"; name="{}""#,&attachment.2,&attachment.0));
        build.push_str(&"\r\n");
        build.push_str(&format!(r#"Content-Disposition: attachment; filename="{}""#,&attachment.0));
        build.push_str(&"\r\n");
        build.push_str(&format!("Content-Transfer-Encoding: base64\r\n\r\n"));
        build.push_str(&format!("{}\r\n",build_line(attachment.1.clone())));
    }
    return build;
}

fn unique_id(size:usize)->String{
    let rand_string: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(size)
    .map(char::from)
    .collect();
    return rand_string;
}

fn hash_sha256(v:String) -> String{
    let mut hasher = Sha256::new();
    // println!("{}",v);
    hasher.update(&v.into_bytes());
    let hash = hasher.finish();
    let encode = base64::encode(&hash);
    return encode;
}

fn sign_here(private_key:&PKey<Private>,val:Vec<u8>) -> Result<String,&'static str>{

    let encoded_signature:String;
    match Signer::new(MessageDigest::sha256(), &private_key){
        Ok(mut signer)=>{
            match signer.update(&val){
                Ok(_)=>{
                    match signer.sign_to_vec(){
                        Ok(signature)=>{
                            encoded_signature = base64::encode(&signature);
                        },
                        Err(_)=>{return Err("failed-sign");}
                    }
                },
                Err(_)=>{return Err("failed-insert_data_into_signer");}
            }
        },
        Err(_)=>{return Err("failed-initiate_signer");}
    }

    return Ok(encoded_signature);

}