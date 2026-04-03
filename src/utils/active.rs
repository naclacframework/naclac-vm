use crate::config::Config;
use std::fs;

pub fn get_active_version(config: &Config) -> Option<String> {
    #[cfg(target_family = "unix")]
    {
        let router_path = config.bin_dir.join("naclac");
        if let Ok(target) = fs::read_link(router_path) {
            // The target path is conceptually .../versions/<version>/bin/naclac
            if let Some(parent) = target.parent() {     // .../versions/<version>/bin
                if let Some(grandparent) = parent.parent() { // .../versions/<version>
                    if let Some(version_name) = grandparent.file_name() {
                        return Some(version_name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    #[cfg(target_family = "windows")]
    {
        let cmd_path = config.bin_dir.join("naclac.cmd");
        if let Ok(contents) = fs::read_to_string(cmd_path) {
            if let Some(versions_idx) = contents.find("versions") {
                let remainder = &contents[versions_idx + 9..]; // Skip "versions/" or "versions\"
                let end_idx = remainder.find('/').unwrap_or_else(|| remainder.find('\\').unwrap_or(0));
                if end_idx > 0 {
                    return Some(remainder[..end_idx].to_string());
                }
            }
        }
    }

    None
}
