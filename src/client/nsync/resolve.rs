use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::{ResolverConfig,ResolverOpts};
use trust_dns_resolver::proto::rr::rdata::mx::MX;

pub struct RESV{
    pub base:String,
    pub pool:Vec<MX>
}

pub async fn init(email:String) -> Result<RESV,&'static str>{

    if email.contains("localhost"){return Ok(RESV {base:"127.0.0.1:25".to_string(),pool:Vec::new()});}
    if !email.contains("@") || !email.contains("."){return Err("invalid_email");}

    let hold = email.split("@").collect::<Vec<&str>>();
    if hold.len() != 2{return Err("invalid_email");}
    let domain = hold[1];
    if !domain.contains("."){return Err("invalid_domain");}

    match AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()).await{
        Ok(resolver)=>{

            match resolver.mx_lookup(domain).await{
                Ok(lookup)=>{
                    let mut pool:Vec<MX> = Vec::new();
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
                    return Ok(RESV {base:"".to_string(),pool:pool});
                },
                Err(e)=>{
                    println!("!!! failed-lookup-mx_records : {:?}",e);
                    return Err("lookup_failed");
                }
            }

        },
        Err(_)=>{
            return Err("failed-make_resolver");
        }
    }



}
