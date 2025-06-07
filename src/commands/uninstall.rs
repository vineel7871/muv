// src/commands/uninstall.rs
use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_uninstall(args: PackageManagementArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.env_name)?;
    println!(
        "Uninstalling package(s) [{}] from environment '{}'...",
        args.packages.join(", "), args.env_name
    );

    let mut uv_cmd_args = vec!["pip", "uninstall"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // Add -y to auto-confirm uninstall for uv pip uninstall
    uv_cmd_args.push("-y"); 
    
    utils::run_uv_command(&uv_cmd_args, None, vec![("VIRTUAL_ENV", env_path.as_path())])?;

    println!("Package(s) uninstalled successfully from '{}'.", args.env_name);
    Ok(())
}