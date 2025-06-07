// src/commands/remove.rs
use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_remove(args: PackageManagementArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.env_name)?;
    println!(
        "Removing package(s) [{}] from environment '{}' pyproject.toml and uninstalling...",
        args.packages.join(", "), args.env_name
    );
    
    let mut uv_cmd_args = vec!["remove"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // uv remove operates on pyproject.toml in current_dir
    utils::run_uv_command(&uv_cmd_args, Some(&env_path), vec![])?;

    println!("Package(s) removed and uninstalled successfully from '{}'.", args.env_name);
    Ok(())
}