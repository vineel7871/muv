use crate::utils;
use anyhow::Result;

pub fn handle_list() -> Result<()> {
    let envs_dir = utils::get_envs_dir()?;
    println!("Available GUV environments (in {}):", envs_dir.display());

    let mut count = 0;
    if envs_dir.exists() {
        for entry in std::fs::read_dir(envs_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // A simple check: does it look like a venv? (e.g., has pyvenv.cfg)
                if path.join("pyvenv.cfg").exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        println!("- {}", name);
                        count += 1;
                    }
                }
            }
        }
    }
    if count == 0 {
        println!("No environments found. Use 'guv create <name>' to create one.");
    }
    Ok(())
}
