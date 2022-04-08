use git2::Repository;
use std::{fs, env};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Component {
    name: String,
    pull_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentManager {
    components: Vec<Component>
}

const MODULE_INSTALL_PATH: &str = "yacs_modules";

impl ComponentManager {
    pub fn new_default() -> ComponentManager {
        ComponentManager {
            components: vec![
                Component {
                    name: String::from("TestServer2"),
                    pull_url: String::from("https://github.com/xypine/Kirjat.ml-api.git")
                }
            ]
        }
    }
    pub fn new_from_file(path: String) -> Option<ComponentManager> {
        let s = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                return None;
            },
        };
        let deserialized = toml::from_str(&s);
        match deserialized {
            Ok(comp) => Some(comp),
            Err(_) => None,
        }
    }
    pub fn to_file(&self, path: String) {
        let serialized = toml::to_string(self).unwrap();
        fs::write(path, serialized).expect("Failed to write component file");
    }
    pub fn update_components(&self) {
        let app_dir = env::current_dir().unwrap();
        let install_dir = app_dir.join(MODULE_INSTALL_PATH);
        println!("Removing previous module files...");
        fs::remove_dir_all(install_dir.clone());
        println!("Previous files removed!");
        println!("Cloning new files...");
        for c in &self.components {
            let name = &c.name;
            match Repository::clone(&c.pull_url, install_dir.join(name).as_path()) {
                Ok(_) => println!("{} cloned succesfully", name),
                Err(e) => println!("failed to clone: {}", e),
            };
        }
        println!("Update complete!");
    }
}