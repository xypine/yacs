use git2::Repository;
use std::{fs, env, path::PathBuf, process::Command, vec};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Component {
    name: String,
    pull_url: String,
    run: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComponentManager {
    components: Vec<Component>
}

const MODULE_INSTALL_PATH: &str = "yacs_modules";

fn get_yacs_path() -> PathBuf {
    // let exec_path = env::current_exe().unwrap();
    // let actual_path = exec_path.parent().unwrap();
    // return actual_path.to_path_buf();
    env::current_dir().unwrap()
}

impl ComponentManager {
    pub fn new_default() -> ComponentManager {
        ComponentManager {
            components: vec![
                Component {
                    name: String::from("TestServer2"),
                    pull_url: String::from("https://github.com/xypine/Kirjat.ml-api.git"),
                    run: String::from("python3 -m http.server")
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
        match fs::remove_dir_all(install_dir.clone()) {
            Ok(_) => {
                println!("Previous files removed!");
            },
            Err(_) => {
                println!("Couldn't remove previous files, this is normal if no previous exist.");
            },
        }
        println!("Cloning new files...");
        for c in &self.components {
            let name = &c.name;
            match Repository::clone(&c.pull_url, install_dir.join(name).as_path()) {
                Ok(_) => println!("\t{} cloned succesfully", name),
                Err(e) => println!("\tfailed to clone: {}", e),
            };
        }
        println!("Update complete!");
    }
    pub fn run_components(&self) {
        let app_dir = get_yacs_path();
        let install_dir = app_dir.join(MODULE_INSTALL_PATH);
        for c in &self.components {
            let name = &c.name;
            let run = &c.run;
            let run_parts: Vec<&str> = run.split(" ").collect();
            let run_program = *run_parts.get(0).expect("Malformed run parameter");
            let mut args: Vec<&str> = vec![];
            if run_parts.len() > 1 {
                args = run_parts[1..run_parts.len()].to_vec();
            }
            let comp_path = install_dir.join(name);
            println!("Trying to run '{}' in the directory '{}' with the arguments '{:?}'", run_program, comp_path.display(), args);
            let output = Command::new(run_program)
                .current_dir(comp_path)
                .args(args)
                .output()
                .expect(&format!("failed to execute process for {}", name));
            println!("Status:\t{}", output.status);
            println!("Output:\n===\n{}\n===", std::str::from_utf8(&output.stdout).unwrap());
            println!("Errors:\n===\n{}\n===", std::str::from_utf8(&output.stderr).unwrap());
        }
    }
}