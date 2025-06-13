use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_add(args: PackageManagementArgs) -> Result<()> {
    // Get the environment path and name, prioritizing active env
    let (env_path, env_name) = utils::get_active_or_specified_env(args.env_name.as_ref())?;

    println!(
        "Adding package(s) [{}] to environment '{}' pyproject.toml and installing...",
        args.packages.join(", "), env_name
    );

    let mut uv_cmd_args = vec!["add"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // uv add operates on pyproject.toml in current_dir
    utils::run_uv_command(&uv_cmd_args, Some(&env_path), vec![])?;

    println!("Package(s) added and installed successfully in '{}'.", env_name);
    Ok(())
}