mod install;
use install::ins;
mod bdd;
use bdd::make_bdd;
use std::io::stdin;

fn main(){
    println!("Bienvenue sur l'installateur d'environnement de développement.");
    match make_bdd(){
        Ok(())=>{
            println!("Bdd faite.");
        },
        Err(err)=>{
            println!("Erreur à la création de la base de données : {}",err)
        }
    }
    println!("Veuillez spécifier un emplacement pour l'installation");
    let mut l = String::new();
    if let Ok(_x) = stdin().read_line(&mut l){
        match ins::<'b>{    // A revoir, on doit gérer la transition Result vers un Struct Process
            true=>{
                println!("L'installation est terminée.")
            },
            false=>{
                println!("Erreur : {}",x);
                println!("Mise en arrêt du programme.");
            }
        }
    }
    else{
        panic!("Erreur système. Mise en arrêt du programme.")
    }
}