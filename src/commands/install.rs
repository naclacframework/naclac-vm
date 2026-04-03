use crate::config::Config;
use std::process::Command;
use colored::*;

pub fn execute(config: &Config, version: &str) {
    println!("{} naclac v{}...", "Installing".green().bold(), version);

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

    match status {
        Ok(s) if s.success() => {
            println!("{} installed successfully at {:?}", "Version".green().bold(), root_path);
        }
        Ok(s) => {
            println!("{} Cargo exit code {}", "Failed:".red().bold(), s);
        }
        Err(e) => {
            println!("{} {:?}", "Failed to execute cargo:".red().bold(), e);
        }
    }
}
