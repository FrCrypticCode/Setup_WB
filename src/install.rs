use std::collections::HashMap;
use std::process::Command;
use std::env;
use std::fs::{self, OpenOptions};

struct Process<'a>{
    dir: HashMap<&'a str,bool>,
    files: HashMap<&'a str,bool>,
    ts: bool,
    ed_files: HashMap<&'a str,bool>
}

pub fn ins<'b>(x:String)->Process<'b>{
    let mut dir:HashMap<&str, bool> = HashMap::new(); // Gestion création dossiers
    dir.insert("public", make_dir((x.clone()+"/public").as_str()));
    dir.insert("public/scripts", make_dir((x.clone()+"/public/scripts").as_str()));
    dir.insert("public/styles", make_dir((x.clone()+"/public/styles").as_str()));
    dir.insert("public/ressources", make_dir((x.clone()+"/public/ressources").as_str()));

    let mut files:HashMap<&str, bool> = HashMap::new(); // Gestion création fichiers
    files.insert("index.html", make_file((x.clone()+"/public/index.html").as_str())); 
    files.insert("main.js", make_file((x.clone()+"/public/scripts/main.js").as_str()));
    files.insert("main.css", make_file((x.clone()+"/public/styles/main.css").as_str()));
    files.insert("main.ts", make_file((x.clone()+"/main.ts").as_str()));
    files.insert("server.ts", make_file((x.clone()+"/server.ts").as_str()));
    files.insert("tsconfig.json", make_file((x.clone()+"/tsconfig.json").as_str()));

    // Intégration Typescript
    let ts = launch_ts(&x);

    // Implémenter l'écriture dans les fichiers de base
    let edit:HashMap<&str, bool> = HashMap::new();
    //prepare_files(&x){
    return Process { dir: dir, files: files, ts: ts, ed_files: edit }
}

fn make_file(path:&str)->bool{
    if let Err(err) = fs::OpenOptions::new().write(true).read(true).create(true).open(path){
        return false
    }
    return true
}

fn make_dir(path:&str)->bool{
    if let Err(err) = fs::create_dir(path){
        if !err.to_string().contains("error 183"){
            return false
        }
    }
    return true
}

fn prepare_files(path:&String)->Result<(),String>{
    {
        match OpenOptions::new().append(true).write(true).open(path.clone()+"/public/index.html"){
            Ok(file)=>{
                //file.write_all(buf);
            },
            Err(err)=>{return Err(err.to_string())}
        }
    }
    Ok(())
}

fn launch_ts(path:&String)->bool{  // Implémenter une possible intégration de retour d'erreurs
    let  mut cmd = Command::new("cmd");
    let p = check_path(path);
    cmd.current_dir(p);
    match cmd.arg("/C").arg("npm").arg("install").arg("typescript").output(){
        Ok(y)=>{
            if y.status.success(){
                return true
            }
            else{
                return false
            }
        },
        Err(err)=>{
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