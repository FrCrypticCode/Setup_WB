use std::collections::HashMap;

// GUI Structs
pub struct DataBdd{
    pub user: String,
    pub password: String,
    pub ip: String
}
impl Default for DataBdd{
    fn default() -> Self {
        return DataBdd{
            user: String::new(),
            password:String::new(),
            ip:String::new()
        }
    }
}

pub struct AppConfig{
    pub path: String,
    pub err_path: bool,
    ad_err_path: String,
    pub err_bdd: bool,
    add_err_bdd: String
}
impl Default for AppConfig {
    fn default() -> Self {
        return AppConfig { 
            path: String::new(), 
            err_path: false, 
            ad_err_path: String::from("Input path is empty."), 
            err_bdd: false, 
            add_err_bdd: String::from("One or more inputs bdd is empty.") 
        }
    }
}
impl AppConfig{
    pub fn call_err_p(&self)->String{
        return self.ad_err_path.clone();
    }
    pub fn call_err_bdd(&self)->String{
        return self.add_err_bdd.clone();
    }
}

// Logs Struct
pub struct Process<'a>{
    pub dir: HashMap<&'a str,bool>,
    pub files: HashMap<&'a str,bool>,
    pub ts: bool,
    pub ed_files: HashMap<&'a str,bool>,
    pub bdd: bool,
    pub list_err:Vec<String>
}