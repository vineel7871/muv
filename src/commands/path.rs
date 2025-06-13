use crate::utils;
use crate::EnvNameArg;
use anyhow::Result;

pub fn handle_path(args: EnvNameArg) -> Result<()> {
    let (env_path, env_name) = utils::get_active_or_specified_env(args.name.as_ref())?;
    println!("{}", env_path.display());
    Ok(())
}