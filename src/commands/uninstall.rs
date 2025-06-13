use crate::PackageManagementArgs;
use crate::utils;
use anyhow::Result;

pub fn handle_uninstall(args: PackageManagementArgs) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.env_name.as_ref())?;
    println!(
        "Uninstalling package(s) [{}] from environment '{}'...",
        args.packages.join(", "),
        env_name
    );

    let mut uv_cmd_args = vec!["pip", "uninstall"];
    for pkg in &args.packages {
        uv_cmd_args.push(pkg.as_str());
    }
    // Add -y to auto-confirm uninstall for uv pip uninstall
    // uv_cmd_args.push("-y");

    utils::run_uv_command(
        &uv_cmd_args,
        None,
        vec![("VIRTUAL_ENV", env_path.as_path())],
    )?;

    println!("Package(s) uninstalled successfully from '{}'.", env_name);
    Ok(())
}
