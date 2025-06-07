// src/commands/install.rs
use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_install(args: PackageManagementArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.env_name)?;
    println!(
        "Installing package(s) [{}] into environment '{}'...",
        args.packages.join(", "), args.env_name
    );

    let mut uv_cmd_args = vec!["pip", "install"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    
    // Tell uv to use this specific virtual environment
    utils::run_uv_command(&uv_cmd_args, None, vec![("VIRTUAL_ENV", env_path.as_path())])?;

    println!("Package(s) installed successfully in '{}'.", args.env_name);
    Ok(())
}