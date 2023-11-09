extern crate mysql;
use mysql::{*, prelude::Queryable};

pub fn make_bdd()->Result<(),Error>{
    let opts = Opts::from(
        OptsBuilder::new()
        .user(Some("root"))
        .pass(Some(""))
        .db_name(None::<&str>)
        .ip_or_hostname(Some("127.0.0.1"))
    );
    match make_pool(opts){
        Ok(p)=>{
            match p.get_conn(){
                Ok(mut x)=>{
                    let q = "CREATE DATABASE IF NOT EXISTS awesome".to_string();
                match x.query::<String,String>(q){
                    Ok(_x)=>{Ok(())},
                    Err(err)=>{return Err(err)}
                }
                },
                Err(err)=>{
                    return Err(err)
                }
            }            
        },
        Err(err)=>{
            Err(err)
        }
    }
}

fn make_pool(x:Opts)->Result<Pool,Error>{
    match Pool::new(x){
        Ok(p)=>{return Ok(p)},
        Err(err)=>{return Err(err)}
    }
}