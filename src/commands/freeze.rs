// src/commands/freeze.rs
use crate::utils;
use crate::EnvNameArg;
use anyhow::Result;

pub fn handle_freeze(args: EnvNameArg) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.name)?;
    // println!("Installed packages in environment '{}':", args.name);

    let output = utils::get_command_output("uv", &["pip", "freeze"], None, vec![("VIRTUAL_ENV", env_path.as_path())])?;
    print!("{}", output); // Output already has newlines
    Ok(())
}