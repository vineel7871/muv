// src/commands/path.rs
use crate::utils;
use crate::EnvNameArg;
use anyhow::Result;

pub fn handle_path(args: EnvNameArg) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.name)?;
    println!("{}", env_path.display());
    Ok(())
}