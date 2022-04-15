use std::{env, io, fs, process::Command};
use std::{thread, time};
use std::io::Write;
use std::path::PathBuf;

macro_rules! FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_IN { () => { "../files/systemd/yacs_updater.service" } }
const FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_OUT: &str = "/etc/systemd/system/yacs_updater.service";
const FILENAME_SYSTEMD_COMPONENTUPDATER: &str = "yacs_updater.service";

macro_rules! FILENAME_SYSTEMD_COMPONENTRUNNER_SERVICE_IN { () => { "../files/systemd/yacs_runner.service" } }
const FILENAME_SYSTEMD_COMPONENTRUNNER_SERVICE_OUT: &str = "/etc/systemd/system/yacs_runner.service";
const FILENAME_SYSTEMD_COMPONENTRUNNER: &str = "yacs_runner.service";

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

fn write_systemd_update_service(write_path: &str, executable_path: PathBuf) -> Result<(), io::Error> {
    let generated = format!(include_str!(FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_IN!()), executable_path.display());
    println!("DATA ===");
    println!("{}", generated);
    println!("===");
    fs::write(write_path, generated)?;
    Ok(())
}

fn write_systemd_runner_service(write_path: &str, executable_path: PathBuf) -> Result<(), io::Error> {
    let generated = format!(include_str!(FILENAME_SYSTEMD_COMPONENTRUNNER_SERVICE_IN!()), executable_path.display());
    println!("DATA ===");
    println!("{}", generated);
    println!("===");
    fs::write(write_path, generated)?;
    Ok(())
}

pub fn install(skip_warn: bool) -> Result<(), io::Error> {
    banner();
    if skip_warn {
        println!("You have chosen to skip warnings and any additional confirmations. (--yes)");
    }
    else {
        warning_countdown();
    }
    let app_dir = env::current_dir().unwrap();
    let app_executable = env::current_exe().unwrap();
    println!("The current directory is {}", app_dir.display());

    println!("Installing a systemd service file to {}...", FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_OUT);
    write_systemd_update_service(FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_OUT, app_executable.clone())?;
    
    println!("Installing a systemd service file to {}...", FILENAME_SYSTEMD_COMPONENTUPDATER_SERVICE_OUT);
    write_systemd_runner_service(FILENAME_SYSTEMD_COMPONENTRUNNER_SERVICE_OUT, app_executable)?;

    println!("Enabling systemd services...");
    let errmsg1 = format!("Couldn't enable service \"{}\"", FILENAME_SYSTEMD_COMPONENTUPDATER);
    let out1 = Command::new("systemctl")
                .args(["enable", FILENAME_SYSTEMD_COMPONENTUPDATER])
                .output()
                .expect(&errmsg1);
    println!("Enable output: {}", std::str::from_utf8(&out1.stdout).unwrap());
    println!("Enable errors: {}", std::str::from_utf8(&out1.stderr).unwrap());
    let errmsg2 = format!("Couldn't enable service \"{}\"", FILENAME_SYSTEMD_COMPONENTRUNNER);
    let out2 = Command::new("systemctl")
                .args(["enable", FILENAME_SYSTEMD_COMPONENTRUNNER])
                .output()
                .expect(&errmsg2);
    println!("Enable output: {}", std::str::from_utf8(&out2.stdout).unwrap());
    println!("Enable errors: {}", std::str::from_utf8(&out2.stderr).unwrap());


    println!("Starting systemd services...");
    let errmsg1 = format!("Couldn't start service \"{}\"", FILENAME_SYSTEMD_COMPONENTUPDATER);
    let out1 = Command::new("systemctl")
                .args(["start", FILENAME_SYSTEMD_COMPONENTUPDATER])
                .output()
                .expect(&errmsg1);
    println!("Start output: {}", std::str::from_utf8(&out1.stdout).unwrap());
    println!("Start errors: {}", std::str::from_utf8(&out1.stderr).unwrap());
    let errmsg2 = format!("Couldn't start service \"{}\"", FILENAME_SYSTEMD_COMPONENTRUNNER);
    let out2 = Command::new("systemctl")
                .args(["start", FILENAME_SYSTEMD_COMPONENTRUNNER])
                .output()
                .expect(&errmsg2);
    println!("Start output: {}", std::str::from_utf8(&out2.stdout).unwrap());
    println!("Start errors: {}", std::str::from_utf8(&out2.stderr).unwrap());

    Ok(())
}