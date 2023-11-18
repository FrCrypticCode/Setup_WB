mod install;
use egui::Color32;
use install::ins;
mod bdd;

use eframe::{self, NativeOptions};
use std::sync::{Arc,Mutex};

mod obj;
use obj::{AppConfig,DataBdd};

fn main(){
    let opts = NativeOptions{
        initial_window_size: Some(egui::vec2(480.0,360.0)),
        drag_and_drop_support:true,
        ..Default::default()
    };

    let app_m = Arc::new(Mutex::new(AppConfig::default()));
    let bdd_m = Arc::new(Mutex::new(DataBdd::default()));
    let valid = Arc::new(Mutex::new(false));

    // Duplication des pointeurs
    let app1 = app_m.clone();
    let bdd1 = bdd_m.clone();
    let v = valid.clone();

    let mut mdp = String::new();

    let f_window = eframe::run_simple_native("Setup WebEnv", opts, move |ctx,frame|{
        egui::CentralPanel::default().show(ctx, |ui|{
            // Verrouillage des Structs sur la première fenêtre 
            // - Non sollicité ailleurs => Unwrap safe
            let mut app = app1.lock().unwrap();
            let mut bdd = bdd1.lock().unwrap();
            ui.label("Install WebEnv");
            ui.label("Welcome in our personal setup, please insert the path for the installation :");
            ui.text_edit_singleline(&mut app.path);
            if app.err_path{
                ui.colored_label(Color32::from_rgb(255, 0, 0), app.call_err_p());
            }
            ui.separator();
            // Implémenter le formulaire pour la connexion BDD SQL
            ui.label("SQL BDD Auth");
            ui.horizontal(|ui|{
                ui.label("User : ");
                ui.text_edit_singleline(&mut bdd.user);
            });
            ui.horizontal(|ui|{
                ui.label("Password : ");
                mdp = "*".repeat(bdd.password.len());
                let input = ui.text_edit_singleline(&mut mdp); // Concevoir un mod de vue anonyme
                if input.changed(){
                    if mdp.chars().last() != Some('*'){
                        if mdp.len() != 0{
                            bdd.password.push(mdp.chars().last().unwrap());
                        }
                        else{
                            bdd.password = mdp.clone();
                        }
                        
                    }
                    else if mdp.chars().last() == Some('*'){
                        if bdd.password.len() == 1{
                            bdd.password = String::from("");
                        }
                        else{
                            bdd.password.pop();
                        }    
                    }
                }
            });
            ui.horizontal(|ui|{
                ui.label("IP Address : ");
                ui.text_edit_singleline(&mut bdd.ip);
            });
            if app.err_bdd{
                ui.colored_label(Color32::from_rgb(255, 0, 0), app.call_err_bdd());
            }
            ui.separator();
            ui.horizontal(|ui|{
                if ui.button("Confirm").clicked(){
                    if app.path.len() != 0 && bdd.user.len() != 0{
                        app.err_path = false;
                        app.err_bdd = false;
                        {
                            let mut s = v.lock().unwrap();
                            *s = true;
                        }
                        frame.close();
                    }
                    else if app.path.len() == 0 && (bdd.user.len() == 0||bdd.ip.len() == 0){
                        app.err_path = true;
                        app.err_bdd = true;
                    }
                    else if app.path.len() == 0{
                        app.err_path = true;
                        app.err_bdd = false;
                    }
                    else if bdd.user.len() == 0 || bdd.ip.len() == 0{
                        app.err_path = false;
                        app.err_bdd = true;
                    }
                }
                if ui.button("Abort").clicked(){
                    frame.close();
                }
            });
        });
    });

    let check = valid.clone();
    if *check.lock().unwrap(){
        let app2 = app_m.clone();
        let bdd2 = bdd_m.clone();
        let app = app2.lock().unwrap();
        let bdd = bdd2.lock().unwrap();
        let process = Arc::new(ins(&app.path,bdd));
        let p = process.clone();

        let opts = NativeOptions{
            initial_window_size:Some(egui::vec2(400.0,400.0)),
            ..Default::default()
        };
        let s_window = eframe::run_simple_native("Install Report", opts, move |ctx,frame|{
            egui::CentralPanel::default().show(ctx, |ui|{
                ui.label("Installation Report");
                ui.separator();
                ui.columns(2, |c|{
                    for x in p.dir.to_owned(){
                        let (s,b) = x;
                        c[0].label(s);
                        if b{
                            c[1].colored_label(Color32::from_rgb(10, 220, 10), "Complete !");
                        }else{
                            c[1].colored_label(Color32::from_rgb(220, 10, 10), "Not complete.");
                        }                        
                    }
                    for x in p.files.to_owned(){
                        let (s,b) = x;
                        c[0].label(s);
                        if b{
                            c[1].colored_label(Color32::from_rgb(10, 220, 10), "Complete !");
                        }else{
                            c[1].colored_label(Color32::from_rgb(220, 10, 10), "Not complete.");
                        }                        
                    }
                    for x in p.ed_files.to_owned(){
                        let (s,b) = x;
                        c[0].label(s);
                        if b{
                            c[1].colored_label(Color32::from_rgb(10, 220, 10), "Complete !");
                        }else{
                            c[1].colored_label(Color32::from_rgb(220, 10, 10), "Not complete.");
                        } 
                    }
                    c[0].label("Typescript");
                    if process.ts{
                        c[1].colored_label(Color32::from_rgb(10, 220, 10), "Complete !");
                    }else{
                        c[1].colored_label(Color32::from_rgb(220, 10, 10), "Not complete.");
                    }
                    c[0].label("Database");
                    if process.bdd{
                        c[1].colored_label(Color32::from_rgb(10, 220, 10), "Complete !");
                    }else{
                        c[1].colored_label(Color32::from_rgb(220, 10, 10), "Not complete.");
                    } 
                });
                ui.separator();
                for x in process.list_err.to_owned(){
                    ui.label(x);
                }
                ui.vertical_centered(|ui|{
                    if ui.button("Quit").clicked(){
                        frame.close();
                    }
                });
            });
        });

        match f_window{
            Ok(_x)=>{},
            Err(err)=>{panic!("{err}")}
        }
        match s_window{
            Ok(_x)=>{},
            Err(err)=>{panic!("{err}")}
        }
    }
}

// Problème Copy function
