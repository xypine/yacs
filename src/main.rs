pub mod installer;
use installer::install;
pub mod components;
use components::ComponentManager;

use clap::{Parser, Subcommand};

/// Deploy any stateless app on a node instantly
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {

    /// Skip nonfatal warnings
    #[clap(short, long, global(true))]
    yes: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install yacs
    Install{  },
    /// Update installed components. DELETES ALL PREVIOUS COMPONENT DATA
    UpdateComponents{  },
    /// Run installed components.
    RunComponents{  },
}

fn get_componentmanager() -> ComponentManager {
    let comp = match ComponentManager::new_from_file("yacs_components.toml".to_string()) {
        Some(comp) => comp,
        None => {
            println!("Creating missing component file yacs_components.toml...");
            ComponentManager::new_default()
        },
    };
    comp.to_file("yacs_components.toml".to_string());
    comp
}

fn main() {
    let args = Args::parse();

    let skip_warn = args.yes;

    match &args.command {
        Commands::UpdateComponents {  } => {
            println!("Updating components...");
            
            let comp = get_componentmanager();
            comp.update_components();
        },
        Commands::RunComponents {  } => {
            println!("Starting components...");
            
            let comp = get_componentmanager();
            comp.run_components(true);
        },
        _ => { // Also Commands::Install (default)
            install(skip_warn).expect("Installation failed");
        }
    }

}
