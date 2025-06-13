use crate::utils;
use crate::PackageManagementArgs;
use anyhow::Result;

pub fn handle_install(args: PackageManagementArgs) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.env_name.as_ref())?;

    println!(
        "Installing package(s) [{}] into environment '{}'...",
        args.packages.join(", "), env_name
    );

    let mut uv_cmd_args = vec!["pip", "install"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }

    // Tell uv to use this specific virtual environment
    utils::run_uv_command(&uv_cmd_args, None, vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())])?;

    println!("Package(s) installed successfully in '{}'.", env_name);
    Ok(())
}