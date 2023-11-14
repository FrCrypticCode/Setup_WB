extern crate mysql;
use mysql::{*, prelude::Queryable};

use std::io::stdin;

pub fn make_bdd(errs:&mut Vec<String>)->bool{
    let mut entry = String::new();
    stdin().read_line(&mut entry).unwrap_or(0);
    let opts = Opts::from(
        OptsBuilder::new()
        .user(Some("root"))
        .pass(Some(entry.as_str()))
        .db_name(None::<&str>)
        .ip_or_hostname(Some("127.0.0.1"))
    );
    match make_pool(opts){
        Ok(p)=>{
            match p.get_conn(){
                Ok(mut x)=>{
                    let q = "CREATE DATABASE IF NOT EXISTS awesome".to_string();
                match x.query::<String,String>(q){
                    Ok(_x)=>{return true},
                    Err(err)=>{
                        errs.push(err.to_string());
                        return false
                    }
                }
                },
                Err(err)=>{
                    errs.push(err.to_string());
                    return false
                }
            }            
        },
        Err(_err)=>{
            return false
        }
    }
}

fn make_pool(x:Opts)->Result<Pool,Error>{
    match Pool::new(x){
        Ok(p)=>{return Ok(p)},
        Err(err)=>{return Err(err)}
    }
}