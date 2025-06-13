use crate::utils;
use crate::EnvNameArg;
use anyhow::Result;

pub fn handle_freeze(args: EnvNameArg) -> Result<()> {
    let (env_path, _env_name) = utils::get_active_or_specified_env(args.name.as_ref())?;
    // println!("Installed packages in environment '{}':", env_name); // Optional: if you want to print the name

    let output = utils::get_command_output(
        "uv",
        &["pip", "freeze"],
        None, // current_dir
        vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())] // env_vars
    )?;
    print!("{}", output); // Output already has newlines
    Ok(())
}