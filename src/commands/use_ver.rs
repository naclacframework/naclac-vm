use crate::config::Config;
use crate::utils::router;
use std::env;
use colored::*;

pub fn execute(config: &Config, version: &str) {
    let root_path = config.versions_dir.join(version);
    
    let bin_name = format!("naclac{}", env::consts::EXE_SUFFIX);
    let bin_path = root_path.join("bin").join(&bin_name);

    if !bin_path.exists() {
        println!("{} Version {} is not installed. Run `nacvm install {}` first.", "Error:".red().bold(), version, version);
        return;
    }

    match router::create_router(config, version) {
        Ok(_) => {
            println!("{} Using version {}", "Success:".green().bold(), version);
        }
        Err(e) => {
            println!("{} Failed to update active version: {}", "Error:".red().bold(), e);
        }
    }
}
