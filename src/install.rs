use std::collections::HashMap;
use std::process::Command;
use std::env;
use std::fs::{self, OpenOptions, copy};

use crate::bdd::make_bdd;

pub struct Process<'a>{
    pub dir: HashMap<&'a str,bool>,
    pub files: HashMap<&'a str,bool>,
    pub ts: bool,
    pub ed_files: HashMap<&'a str,bool>,
    pub bdd: bool,
    pub list_err:Vec<String>
}

pub fn ins<'b>(x:String)->Process<'b>{
    let mut l_err:Vec<String> = vec![];
    let mut dir:HashMap<&str, bool> = HashMap::new(); // Gestion création dossiers
    dir.insert("public", make_dir((x.clone()+"/public").as_str(),&mut l_err));
    dir.insert("public/scripts", make_dir((x.clone()+"/public/scripts").as_str(),&mut l_err));
    dir.insert("public/styles", make_dir((x.clone()+"/public/styles").as_str(),&mut l_err));
    dir.insert("public/ressources", make_dir((x.clone()+"/public/ressources").as_str(),&mut l_err));

    let mut files:HashMap<&str, bool> = HashMap::new(); // Gestion création fichiers
    files.insert("index.html", make_file((x.clone()+"/public/index.html").as_str(),&mut l_err)); 
    files.insert("main.js", make_file((x.clone()+"/public/scripts/main.js").as_str(),&mut l_err));
    files.insert("main.css", make_file((x.clone()+"/public/styles/main.css").as_str(),&mut l_err));
    files.insert("main.ts", make_file((x.clone()+"/main.ts").as_str(),&mut l_err));
    files.insert("server.ts", make_file((x.clone()+"/server.ts").as_str(),&mut l_err));
    files.insert("tsconfig.json", make_file((x.clone()+"/tsconfig.json").as_str(),&mut l_err));

    // Intégration Typescript
    let ts = launch_ts(&x,&mut l_err);

    // Implémenter l'écriture dans les fichiers de base - Fonction adaptable à 3 arguments
    let mut edit:HashMap<&str, bool> = HashMap::new();
    edit.insert("index.html",prepare_file(&x,"index.html","",&mut l_err));
    edit.insert("main.css",prepare_file(&x,"main.css","styles",&mut l_err));
    edit.insert("main.js",prepare_file(&x,"main.js","scripts",&mut l_err));
    edit.insert("tsconfig.json",prepare_file(&x,"tsconfig.json","",&mut l_err));
    
    // Conception base de données
    let bdd = make_bdd(&mut l_err);

    return Process { dir: dir, files: files, ts: ts, ed_files: edit, bdd:bdd, list_err:l_err }
}

fn make_file(path:&str,errs:&mut Vec<String>)->bool{
    if let Err(err) = fs::OpenOptions::new().write(true).read(true).create(true).open(path){
        errs.push(err.to_string());
        return false
    }
    return true
}

fn make_dir(path:&str,errs:&mut Vec<String>)->bool{
    if let Err(err) = fs::create_dir(path){
        if !err.to_string().contains("error 183"){
            errs.push(err.to_string());
            return false
        }
    }
    return true
}

fn prepare_file(path:&String,file:&str,dir:&str,errs:&mut Vec<String>)->bool{
    let p:String;
    if dir.len() != 0{
        p = path.clone()+"/public/"+dir+"/"+file;
    }
    else{
        p = path.clone()+"/public/"+file;
    }
    match OpenOptions::new().read(true).write(true).open(&p){
        Ok(_f)=>{
            let source = String::from("res/")+file;
            match copy(source, p){
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
}

fn launch_ts(path:&String,errs:&mut Vec<String>)->bool{  // Implémenter une possible intégration de retour d'erreurs
    let  mut cmd = Command::new("cmd");
    let p = check_path(path);
    cmd.current_dir(p);
    match cmd.arg("/C").arg("npm").arg("install").arg("typescript").output(){
        Ok(y)=>{
            if y.status.success(){
                return true
            }
            else{
                errs.push("Erreur de statut".to_string());
                return false
            }
        },
        Err(err)=>{
            errs.push(err.to_string());
            return false
        }
    }
}

fn check_path(path:&String)->String{
    if path.starts_with('/'){
        return String::from(env::current_dir().unwrap().to_string_lossy())+path;
    }
    else if path.starts_with("./"){
        let path = &path[1..];
        return String::from(env::current_dir().unwrap().to_string_lossy())+path;
    }
    else{
        return String::from(env::current_dir().unwrap().to_string_lossy())+"/"+path;
    }
}