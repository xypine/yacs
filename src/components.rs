use std::{fs, process::{Command, Stdio}, vec, thread::{self, JoinHandle}, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::util::{get_yacs_path, get_yacs_exec_path};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PathConfig {
    main: PathBuf,
    exec: PathBuf
}

impl PathConfig {
    pub fn new() -> PathConfig {
        PathConfig {
            main: get_yacs_path(),
            exec: get_yacs_exec_path()
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Component {
    name: String,
    pull_url: String,
    run: Vec<String>,
    run_after_update: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ComponentManager {
    components: Vec<Component>,
    paths: PathConfig
}

const MODULE_INSTALL_PATH: &str = "yacs_modules";
const MODULE_INSTALL_SOURCE_PATH: &str = "source";
const MODULE_INSTALL_LIVE_PATH: &str = "live";

impl ComponentManager {
    pub fn new_default() -> ComponentManager {
        ComponentManager {
            components: vec![
                Component {
                    name: String::from("YACSMS"),
                    pull_url: String::from("https://github.com/xypine/yacs_manager.git"),
                    run: vec![String::from("sh runme.sh")],
                    run_after_update: vec![]
                }
            ],
            paths: PathConfig::new()

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
        let app_dir = get_yacs_path();
        let install_dir = app_dir.join(MODULE_INSTALL_PATH).join(MODULE_INSTALL_SOURCE_PATH);
        println!("Removing previous module files...");
        match fs::remove_dir_all(install_dir.clone()) {
            Ok(_) => {
                println!("Previous files removed!");
            },
            Err(_) => {
                println!("Couldn't remove previous files, this is normal if no previous exist.");
            },
        }
        println!("Creating new source directory...");
        match fs::create_dir_all(install_dir.clone()) {
            Ok(_) => {
                println!("Source directory created!");
            },
            Err(_) => {
                println!("Couldn't create source directory!");
            },
        }
        println!("Cloning new files...");
        for c in &self.components {
            let name = &c.name;
            println!("Cloning \"{}\" from \"{}\" into \"{}\"", name, c.pull_url, install_dir.display());
            let cloneout = Command::new("git")
                        .current_dir(install_dir.clone())
                        .args(["clone", &c.pull_url, name])
                        .output()
                        .expect("Couldn't clone repo files");
            println!("Clone output: {}", std::str::from_utf8(&cloneout.stdout).unwrap());
            println!("Clone errors: {}", std::str::from_utf8(&cloneout.stderr).unwrap());
        }
        println!("Running after-update commands...");

        let components = self.components.clone();
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for c in components {
            let handle = thread::spawn(move || {
                let app_dir = get_yacs_path();
                let install_dir = app_dir.join(MODULE_INSTALL_PATH).join(MODULE_INSTALL_SOURCE_PATH); // Source dir
                
                let name = &c.name;
                let run_list = &c.run_after_update;
                Self::run_commands(run_list, name, install_dir, true);
            });
            handles.push(handle);
        }
        for i in handles {
            i.join().unwrap();
        }

        println!("\nUpdate complete!");
    }
    pub fn run_components(&self, show_output: bool) {

        // Copy all modules from the "source" dir to the "live" dir
        let app_dir = get_yacs_path();
        let target_dir = app_dir.join(MODULE_INSTALL_PATH).join(MODULE_INSTALL_LIVE_PATH);
        println!("Removing previous live module files...");
        match fs::remove_dir_all(target_dir.clone()) {
            Ok(_) => {
                println!("Previous live files removed!");
            },
            Err(_) => {
                println!("Couldn't remove previous live files, this is normal if no previous exist.");
            },
        }

        println!("Copying new files to live from source...");
        let copyout = Command::new("cp")
                        .current_dir(app_dir.join(MODULE_INSTALL_PATH))
                        .args(["-r", MODULE_INSTALL_SOURCE_PATH, MODULE_INSTALL_LIVE_PATH])
                        .output()
                        .expect("Couldn't copy files, remember to download the components before trying to run them (update-components)");
        println!("Copy output: {}", std::str::from_utf8(&copyout.stdout).unwrap());
        println!("Copy errors: {}", std::str::from_utf8(&copyout.stderr).unwrap());

        let components = self.components.clone();
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for c in components {
            let handle = thread::spawn(move || {
                let app_dir = get_yacs_path();
                let install_dir = app_dir.join(MODULE_INSTALL_PATH).join(MODULE_INSTALL_LIVE_PATH); // Live dir
                
                let name = &c.name;
                let run_list = &c.run;
                Self::run_commands(run_list, name, install_dir, show_output);
            });
            handles.push(handle);
        }
        for i in handles {
            i.join().unwrap();
        }
    }

    fn run_commands(run_list: &Vec<String>, name: &String, install_dir: PathBuf, show_output: bool) {
        for run in run_list {
            let run_parts: Vec<&str> = run.split(" ").collect();
            let run_program = *run_parts.get(0).expect("Malformed run parameter");
            let mut args: Vec<&str> = vec![];
            if run_parts.len() > 1 {
                args = run_parts[1..run_parts.len()].to_vec();
            }
            let comp_path = install_dir.join(name);

            let std_out = if show_output { Stdio::inherit() } else { Stdio::null() };
            let std_err = if show_output { Stdio::inherit() } else { Stdio::null() };

            println!("{}\tTrying to run '{}' in the directory '{}' with the arguments '{:?}'", name, run_program, comp_path.display(), args);
            let output = Command::new(run_program)
                .current_dir(comp_path)
                .args(args)
                .stdin(Stdio::null())
                .stdout( std_out )
                .stderr( std_err )
                .output()
                .expect(&format!("failed to execute process for {}. Remember to download the components before trying to run them (update-components)", name));
            println!("{}\tStatus:\t{}", name, output.status);
            println!("{}\tOutput:\n===\n{}\n===", name, std::str::from_utf8(&output.stdout).unwrap());
            println!("{}\tErrors:\n===\n{}\n===", name, std::str::from_utf8(&output.stderr).unwrap());
        }
    }
}
