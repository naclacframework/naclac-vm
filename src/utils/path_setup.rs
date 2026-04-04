use std::path::Path;
use colored::*;

#[cfg(target_family = "unix")]
pub fn ensure_in_path(bin_dir: &Path) {
    use std::fs::OpenOptions;
    use std::io::{Read, Write};

    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return,
    };

    let export_line = format!("\n# Naclac Version Manager\nexport PATH=\"{}:$PATH\"\n", bin_dir.to_string_lossy());
    let mut modified_any = false;

    let shell_configs = vec![".zshrc", ".bashrc", ".bash_profile", ".profile"];

    for config_name in shell_configs {
        let config_path = home_dir.join(config_name);
        
        if config_path.exists() {
            if let Ok(mut file) = std::fs::File::open(&config_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() && contents.contains(".nacvm/bin") {
                    continue; 
                }
            }

            if let Ok(mut file) = OpenOptions::new().append(true).open(&config_path) {
                if file.write_all(export_line.as_bytes()).is_ok() {
                    modified_any = true;
                    println!(
                        "{} Automatically added nacvm to your ~/{}",
                        "Success:".green().bold(),
                        config_name
                    );
                }
            }
        }
    }

    if modified_any {
        println!(
            "{} Please restart your terminal or run `source ~/.zshrc` (or ~/.bashrc) to apply changes.",
            "Note:".yellow().bold()
        );
    }
}

#[cfg(target_family = "windows")]
pub fn ensure_in_path(bin_dir: &Path) {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env_key = match hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE) {
        Ok(key) => key,
        Err(_) => return, // Silently fail if we can't access the registry
    };

    let current_path: String = env_key.get_value("Path").unwrap_or_default();
    let bin_str = bin_dir.to_string_lossy().to_string();

    if !current_path.contains(&bin_str) {
        let new_path = if current_path.ends_with(';') || current_path.is_empty() {
            format!("{}{}", current_path, bin_str)
        } else {
            format!("{};{}", current_path, bin_str)
        };

        if env_key.set_value("Path", &new_path).is_ok() {
            println!(
                "{} Automatically added {:?} to your Windows PATH!",
                "Success:".green().bold(),
                bin_str
            );
            println!(
                "{} You may need to restart your terminal for changes to take effect.",
                "Note:".yellow().bold()
            );
        }
    }
}
