mod install;
use install::ins;
mod bdd;
use std::io::stdin;

fn main(){
    println!("Bienvenue sur l'installateur d'environnement de développement.");
    println!("Veuillez spécifier un emplacement pour l'installation");
    let mut l = String::new();
    if let Ok(_x) = stdin().read_line(&mut l){
        let process = ins(l);
        println!("{:?}",process.dir);
        println!("{:?}",process.files);
        println!("{:?}",process.ed_files);
        println!("{:?}",process.ts);
        println!("{:?}",process.bdd);
        println!("{:?}",process.list_err);
    }
    else{
        panic!("Erreur système. Mise en arrêt du programme.")
    }
}

// Migrer vers du GUI
// Implémenter la config de connexion bdd + Nom de la future bdd
// Exploiter l'Objet Process et son journal d'erreurs intégré
