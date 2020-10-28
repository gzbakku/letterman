use crate::client::Email;

pub fn init(email:&Email,domain:String,message_id:String) -> Result<(),&'static str>{

    let mut compile = String::new();

    compile = add_field(compile,"v".to_string(),"1".to_string());
    compile = add_field(compile,"a".to_string(),"rsa-sha256".to_string());
    compile = add_field(compile,"d".to_string(),domain);
    compile = add_field(compile,"s".to_string(),email.dkim_selector.clone());
    compile = add_field(compile,"h".to_string(),"from:to:subject:date;".to_string());

    println!("{:?}",compile);

    let concantat_header = concantize(email.from.clone(),email.to.clone(),email.subject.clone(),email.date.clone());

    println!("{:?}",concantat_header);

    return Err("no_error");

}

fn add_field(base_in:String,tag:String,value:String) -> String {
    let mut base = base_in;
    if base.len() > 0{
        base.push_str(&"; ".to_string());
    }
    base.push_str(&format!("{}={}",tag,value));
    return base;
}

fn concantize(from:String,to:String,subject:String,date:String) -> String{
    format!(
        "from: {}\r\nto: {}\r\nsubject: {}\r\ndate: {}\r\n"
        ,clean_string(from),clean_string(to),
        clean_string(subject),clean_string(date)
    )
}

fn clean_string(v:String) -> String{
    let hold = v.split(" ").collect::<Vec<&str>>();
    let mut collect = String::new();
    if collect.contains("\t"){
        while collect.contains("\t"){
            collect.replace("\t"," ");
        }
    }
    if collect.contains("\r\n"){
        while collect.contains("\r\n"){
            collect.replace("\r\n","");
        }
    }
    for i in hold{
        if i.len() > 0{
            if collect.len() > 0{
                collect.push_str(&format!(" "));
            }
            collect.push_str(&format!("{}",i));
        }
    }
    if collect.len() == 0{
        collect = " ".to_string();
    }
    return collect;
}
