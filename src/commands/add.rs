// src/commands/add.rs
use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_add(args: PackageManagementArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.env_name)?;
    println!(
        "Adding package(s) [{}] to environment '{}' pyproject.toml and installing...",
        args.packages.join(", "), args.env_name
    );

    let mut uv_cmd_args = vec!["add"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // uv add operates on pyproject.toml in current_dir
    utils::run_uv_command(&uv_cmd_args, Some(&env_path), vec![])?;

    println!("Package(s) added and installed successfully in '{}'.", args.env_name);
    Ok(())
}