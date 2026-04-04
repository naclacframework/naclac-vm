use crate::config::Config;
use std::process::Command;
use std::fs;
use std::env;
use colored::*;

pub fn execute(config: &Config, version: &str) {
    println!("{} naclac v{}... (This may take a few minutes)", "Installing".green().bold(), version);

    let root_path = config.versions_dir.join(version);
    let root_str = root_path.to_string_lossy();

    let status = Command::new("cargo")
        .args(&[
            "install",
            "naclac",
            "--version",
            version,
            "--root",
            &root_str,
        ])
        .status();

    // The path where the binary SHOULD be if compilation was truly successful
    let exe_name = format!("naclac{}", env::consts::EXE_SUFFIX);
    let expected_bin_path = root_path.join("bin").join(&exe_name);

    match status {
        Ok(s) if s.success() && expected_bin_path.exists() => {
            println!("{} installed successfully at {:?}", "Version".green().bold(), root_path);
        }
        _ => {
            // If it failed, was terminated, or the binary is missing, we MUST clean up the dirty folder.
            println!("{} Installation failed or was interrupted. Cleaning up...", "Error:".red().bold());
            let _ = fs::remove_dir_all(&root_path); // Rollback
        }
    }
}
