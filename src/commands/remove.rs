use crate::cli::PackageManagementArgs;
use crate::utils;
use anyhow::Result;

pub fn _handle_remove(args: PackageManagementArgs) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.env_name.as_ref())?;
    println!(
        "Removing package(s) [{}] from environment '{}' pyproject.toml and uninstalling...",
        args.packages.join(", "),
        env_name
    );

    let mut uv_cmd_args = vec!["remove"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // uv remove operates on pyproject.toml in current_dir
    utils::run_uv_command(&uv_cmd_args, Some(&env_path), vec![])?;

    println!(
        "Package(s) removed and uninstalled successfully from '{}'.",
        env_name
    );
    Ok(())
}
