use crate::config::Config;
use std::fs;
use std::io;

#[cfg(target_family = "unix")]
pub fn create_router(config: &Config, version: &str) -> io::Result<()> {
    // 1. Delete ~/.nacvm/bin/naclac
    let router_path = config.bin_dir.join("naclac");
    if router_path.exists() {
        fs::remove_file(&router_path)?;
    }

    // 2. std::os::unix::fs::symlink targeting ~/.nacvm/versions/<v>/bin/naclac
    let target_path = config.versions_dir.join(version).join("bin").join("naclac");
    std::os::unix::fs::symlink(target_path, router_path)
}

#[cfg(target_family = "windows")]
pub fn create_router(config: &Config, version: &str) -> io::Result<()> {
    // 1. Delete ~/.nacvm/bin/naclac.cmd and .ps1 (if exist)
    let cmd_path = config.bin_dir.join("naclac.cmd");
    let ps1_path = config.bin_dir.join("naclac.ps1");

    if cmd_path.exists() {
        fs::remove_file(&cmd_path)?;
    }
    if ps1_path.exists() {
        fs::remove_file(&ps1_path)?;
    }

    // Target executable path inside versions
    let target_path = config.versions_dir.join(version).join("bin").join("naclac.exe");
    let target_str = target_path.to_string_lossy();

    // 2. Write naclac.cmd
    let cmd_contents = format!("@\"{}\" %*\r\n", target_str);
    fs::write(&cmd_path, cmd_contents)?;

    // 3. Write naclac.ps1
    let ps1_contents = format!("& \"{}\" $args\r\n", target_str);
    fs::write(&ps1_path, ps1_contents)?;

    Ok(())
}
