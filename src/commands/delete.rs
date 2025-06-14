use crate::cli::DeleteArgs;
use crate::{error::MuvError, utils};
use anyhow::Result;
use std::io::{self, Write};

pub fn handle_delete(args: DeleteArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.name)?;

    if !args.yes {
        print!(
            "Are you sure you want to delete environment '{}' at {}? [y/N]: ",
            args.name,
            env_path.display()
        );
        io::stdout().flush()?;
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;
        if confirmation.trim().to_lowercase() != "y" {
            // Using anyhow::bail for quick error reporting here
            // Or return Err(GuvError::DeletionNotConfirmed);
            anyhow::bail!(MuvError::DeletionNotConfirmed);
        }
    }

    println!("Deleting environment '{}'...", args.name);
    std::fs::remove_dir_all(&env_path).map_err(MuvError::IoError)?;
    println!("Environment '{}' deleted successfully.", args.name);
    Ok(())
}
