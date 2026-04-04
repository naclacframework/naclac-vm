use crate::config::Config;
use crate::utils::active;
use std::fs;
use colored::*;

pub fn execute(config: &Config, version: &str) {
    let root_path = config.versions_dir.join(version);

    if !root_path.exists() {
        println!("{} Version {} is not installed.", "Error:".red().bold(), version);
        return;
    }

    // 1. Check if the version we are uninstalling is the active one
    let is_active = active::get_active_version(config) == Some(version.to_string());

    // 2. Delete the version folder
    match fs::remove_dir_all(&root_path) {
        Ok(_) => {
            println!("{} Successfully uninstalled version {}", "Success:".green().bold(), version);
            
            // 3. If it was active, wipe the symlink/router so we don't leave ghosts!
            if is_active {
                let bin_path = config.bin_dir.join("naclac");
                let cmd_path = config.bin_dir.join("naclac.cmd");
                let ps1_path = config.bin_dir.join("naclac.ps1");
                
                let _ = fs::remove_file(bin_path);
                let _ = fs::remove_file(cmd_path);
                let _ = fs::remove_file(ps1_path);
                println!("{} The active version was uninstalled. Run `nacvm use <version>` to select a new one.", "Note:".yellow().bold());
            }
        }
        Err(e) => println!("{} Failed to uninstall version {}: {}", "Error:".red().bold(), version, e),
    }
}
