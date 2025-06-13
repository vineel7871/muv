use crate::utils;
use anyhow::Result;

pub fn handle_home() -> Result<()> {
    let home_path = utils::get_guv_home()?;
    println!("{}", home_path.display());
    Ok(())
}
