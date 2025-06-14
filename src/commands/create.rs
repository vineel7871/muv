use crate::cli::CreateArgs;
use crate::{error::MuvError, utils};
use anyhow::Result;
use std::fs;

pub fn handle_create(args: CreateArgs) -> Result<()> {
    let env_name = &args.name;
    let env_path = utils::get_env_path(env_name)?;

    if env_path.exists() {
        // Using anyhow::bail for quick error reporting here
        // Or return Err(GuvError::EnvironmentAlreadyExists(env_name.to_string()));
        anyhow::bail!(MuvError::EnvironmentAlreadyExists(env_name.to_string()));
    }

    fs::create_dir_all(&env_path).map_err(MuvError::IoError)?;
    println!(
        "Creating environment '{}' at {}",
        env_name,
        env_path.display()
    );

    let mut uv_args = vec!["venv"];
    if let Some(python_version) = &args.python {
        uv_args.push("--python");
        uv_args.push(python_version);
    }
    // `uv venv .` or `uv venv <path>` behaviour: if path is specified, it's the target.
    // We want to create it IN env_path, so we can pass env_path as the argument.
    uv_args.push(
        env_path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
    );

    utils::run_uv_command(&uv_args, None, vec![])?;

    // Create a basic pyproject.toml
    // utils::create_basic_pyproject_toml(&env_path)?;
    // println!("Created pyproject.toml in {}", env_path.display());

    // Optional: automatically create a lock file
    // This assumes the pyproject.toml is valid enough for uv to process.
    // println!("Generating lockfile (uv.lock)...");
    // utils::run_uv_command(&["pip", "compile", "pyproject.toml", "-o", "uv.lock"], Some(&env_path), vec![])?;
    // println!("Lockfile created.");

    println!("Environment '{}' created successfully.", env_name);

    if args.packages.len() > 0 {
        println!(
            "Installing package(s) [{}] into environment '{}'...",
            args.packages.join(", "),
            env_name
        );

        let mut uv_cmd_args = vec!["pip", "install"];
        for pkg in &args.packages {
            uv_cmd_args.push(pkg.as_str());
        }

        // Tell uv to use this specific virtual environment
        utils::run_uv_command(
            &uv_cmd_args,
            None,
            vec![(utils::ACTIVE_ENV_VAR, env_path.as_path())],
        )?;

        println!("Package(s) installed successfully in '{}'.", env_name);
    }
    println!("To activate, run: eval \"$(muv activate {})\"", env_name);
    Ok(())
}
