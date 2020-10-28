use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig,ResolverOpts};
use trust_dns_resolver::proto::rr::rdata::mx::MX;

pub struct RESV{
    pub base:String,
    pub pool:Vec<String>
}

pub fn init(email:String) -> Result<RESV,&'static str>{

    if email.contains("localhost"){return Ok(RESV {base:"127.0.0.1:25".to_string(),pool:Vec::new()});}
    if !email.contains("@") || !email.contains("."){return Err("invalid_email");}

    let hold = email.split("@").collect::<Vec<&str>>();
    if hold.len() != 2{return Err("invalid_email");}
    let domain = hold[1];
    if !domain.contains("."){return Err("invalid_domain");}

    let resolver:Resolver;
    match Resolver::new(ResolverConfig::default(), ResolverOpts::default()){
        Ok(r)=>{
            resolver = r;
        },
        Err(_)=>{
            return Err("failed-make_resolver");
        }
    }

    let mut pool:Vec<MX> = Vec::new();
    match resolver.mx_lookup(domain){
        Ok(lookup)=>{
            for a in lookup.iter(){
                if pool.len() == 0 {pool.push(a.clone());} else{
                    if a.preference() > pool[pool.len() - 1].preference(){
                        pool.push(a.clone());
                    } else if a.preference() < pool[0].preference(){
                        pool.insert(0,a.clone());
                    } else {
                        let mut pool_index = 0;
                        for h in &pool{
                            if a.preference() < h.preference(){
                                pool.insert(pool_index,a.clone());break;
                            }
                            pool_index = pool_index + 1;
                        }
                    }
                }
            }
        },
        Err(e)=>{
            println!("!!! failed-lookup-mx_records : {:?}",e);
            return Ok(RESV {base:"".to_string(),pool:vec![domain.to_string()]});
        }
    }

    let mut clean:Vec<String> = Vec::new();
    for i in pool{
        clean.push(parse_domain(&i));
    }

    return Ok(RESV {base:"".to_string(),pool:clean});

}

fn parse_domain(m:&MX) -> String{
    let mut addr = m.exchange().to_utf8();
    if addr.as_bytes()[addr.len()-1] == ".".as_bytes()[0]{
        addr.truncate(&addr.len()-1);   //changed from splitoff to truncate
    }
    return addr.to_string();
}
