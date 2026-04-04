use crate::config::Config;
use crate::utils::active;
use colored::*;
use std::fs;

pub fn execute(config: &Config, version: &str) {
    let root_path = config.versions_dir.join(version);

    if !root_path.exists() {
        println!("{} Version {} is not installed.", "Error:".red().bold(), version);
        return;
    }

    if let Some(active_version) = active::get_active_version(config) {
        if active_version == version {
            println!("{} You are uninstalling the currently active version.", "Warning:".yellow().bold());
            println!("Run `nacvm use <another_version>` to activate a different version.");
        }
    }

    println!("{} naclac v{}...", "Uninstalling".yellow().bold(), version);
    
    match fs::remove_dir_all(&root_path) {
        Ok(_) => {
            println!("{} Successfully uninstalled version {}", "Success:".green().bold(), version);
        }
        Err(e) => {
            println!("{} Failed to uninstall version {}: {}", "Error:".red().bold(), version, e);
        }
    }
}
