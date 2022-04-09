use std::{env, io, fs};
use std::{thread, time};
use std::io::{Write};
use std::path::PathBuf;

macro_rules! FILENAME_SYSTEMD_SERVICE_IN { () => { "../files/systemd/yacs.service" } }
const FILENAME_SYSTEMD_SERVICE_OUT: &str = "/etc/systemd/system/yacs_daemon.service";

fn banner() {
    println!("Welcome to YACS version 0.0.0 installer!");
}

fn warning_countdown() {
    println!("Press ctrl + c to cancel.");
    for i in 0..10 {
        print!(" Starting automated install in {} {}. \r", 10-i, if 10-i == 1 {"second"} else {"seconds"});
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => {
                println!(); // Print a newline to make the previous print visible
            }
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
    println!();
    println!("Starting installation...");
}

fn write_systemd_service(write_path: &str, executable_path: PathBuf) -> Result<(), io::Error> {
    let generated = format!(include_str!(FILENAME_SYSTEMD_SERVICE_IN!()), executable_path.display());
    println!("DATA ===");
    println!("{}", generated);
    println!("===");
    fs::write(write_path, generated)?;
    Ok(())
}

pub fn install(skip_warn: bool) -> Result<(), io::Error> {
    banner();
    if skip_warn {
        println!("You chose to skip warnings. (--yes)");
    }
    else {
        warning_countdown();
    }
    let app_dir = env::current_dir().unwrap();
    let app_executable = env::current_exe().unwrap();
    println!("The current directory is {}", app_dir.display());

    println!("Installing a systemd service file to {}...", FILENAME_SYSTEMD_SERVICE_OUT);
    write_systemd_service(FILENAME_SYSTEMD_SERVICE_OUT, app_executable)?;
    

    Ok(())
}