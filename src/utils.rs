// src/utils.rs
use crate::error::{GuvError, Result}; // Or use anyhow::Result
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;
use std::env;

pub const ACTIVE_ENV_VAR: &str = "VIRTUAL_ENV";
pub const GUV_ACTIVE_ENV_NAME_VAR: &str = "GUV_ENV_NAME";

pub fn get_guv_home() -> Result<PathBuf> {
    let base_dir = dirs::data_dir()
        .or_else(dirs::home_dir)
        .ok_or(GuvError::HomeDirError)?;
    Ok(base_dir.join(".guv"))
}

pub fn get_envs_dir() -> Result<PathBuf> {
    let path = get_guv_home()?.join("envs");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(GuvError::IoError)?;
    }
    Ok(path)
}

pub fn get_env_path(name: &str) -> Result<PathBuf> {
    Ok(get_envs_dir()?.join(name))
}

pub fn ensure_env_exists(name: &str) -> Result<PathBuf> {
    let path = get_env_path(name)?;
    if !path.exists() || !path.join("pyvenv.cfg").exists() { // pyvenv.cfg is a good indicator of a venv
        return Err(GuvError::EnvironmentNotFound(name.to_string()));
    }
    Ok(path)
}

// Check if 'uv' is installed and executable
pub fn check_uv_exists() -> Result<()> {
    Command::new("uv")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| GuvError::UvCommandFailed(format!("Failed to execute uv: {}", e)))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(GuvError::UvNotInstalled("uv".to_string()))
            }
        })
}


// A more generic command runner
pub fn run_uv_command(args: &[&str], current_dir: Option<&Path>, env_vars: Vec<(&str, &Path)>) -> Result<()> {
    let mut cmd = Command::new("uv");
    cmd.args(args);
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
    for (key, val) in env_vars {
        cmd.env(key, val);
    }

    // Optional: Show command being run
    // println!("Executing: uv {}", args.join(" "));
    dbg!(&cmd);
    let status = cmd
        .status()
        .map_err(|e| GuvError::UvCommandFailed(format!("Failed to execute uv: {}", e)))?;
    dbg!(&status);
    if !status.success() {
        let err_msg = format!("uv {} failed with status: {}", args.join(" "), status);
        // You could try to capture stderr here for a better message
        return Err(GuvError::UvCommandFailed(err_msg));
    }
    Ok(())
}

// Helper to get output from a command
pub fn get_command_output(program: &str, args: &[&str], current_dir: Option<&Path>, env_vars: Vec<(&str, &Path)>) -> Result<String> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
     for (key, val) in env_vars {
        cmd.env(key, val);
    }

    let output = cmd
        .output()
        .map_err(|e| GuvError::UvCommandFailed(format!("Failed to execute {}: {}", program, e)))?;

    if !output.status.success() {
        let err_msg = format!(
            "{} {} failed with status: {}. Stderr: {}",
            program,
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(GuvError::UvCommandFailed(err_msg));
    }
    String::from_utf8(output.stdout)
        .map_err(|e| GuvError::UvCommandFailed(format!("Failed to parse output as UTF-8: {}", e)))
}

// Basic pyproject.toml content
pub fn create_basic_pyproject_toml(project_path: &Path) -> Result<()> {
    let toml_content = r#"[project]
name = "guv-environment"
version = "0.1.0"
description = "A Python environment managed by GUV."
requires-python = ">=3.8" # Default, can be made configurable

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

# Add this if you want to use uv for locking and resolution within the env itself
# [tool.uv]
# native-tls = true
"#;
    let toml_file_path = project_path.join("pyproject.toml");
    fs::write(toml_file_path, toml_content).map_err(GuvError::IoError)
}

/// Gets the environment path, prioritizing an active GUV environment,
/// then a specified name, and errors if neither is sufficient.
/// It also returns the determined environment name.
pub fn get_active_or_specified_env(env_name_arg: Option<&String>) -> Result<(PathBuf, String)> {
    // 1. Check for GUV_ENV_NAME and VIRTUAL_ENV (our activation sets these)
    if let (Ok(active_env_path_str), Ok(active_guv_name)) =
        (env::var(ACTIVE_ENV_VAR), env::var(GUV_ACTIVE_ENV_NAME_VAR))
    {
        let active_env_path = PathBuf::from(active_env_path_str);
        // Verify that the VIRTUAL_ENV path seems like a GUV env (e.g., inside get_envs_dir())
        // and that GUV_ENV_NAME matches the basename of VIRTUAL_ENV
        let envs_dir = get_envs_dir()?;
        if active_env_path.starts_with(&envs_dir) &&
           active_env_path.file_name().and_then(|s| s.to_str()) == Some(&active_guv_name) &&
           active_env_path.join("pyvenv.cfg").exists() {

            // If an env_name_arg was also provided, check if it matches the active one.
            // If they don't match, it's ambiguous or a user error.
            if let Some(name_arg) = env_name_arg {
                if name_arg != &active_guv_name {
                    return Err(GuvError::Anyhow(anyhow::anyhow!(
                        "An environment ('{}') is already active, but you specified a different one ('{}').\nDeactivate the current environment or omit the environment name argument.",
                        active_guv_name, name_arg
                    )));
                }
            }
            // If we are here, either no name_arg was given (use active) or it matched active.
            println!("Using active GUV environment: {}", active_guv_name);
            return Ok((active_env_path, active_guv_name));
        } else {
            // VIRTUAL_ENV is set, but it doesn't look like our GUV environment, or GUV_ENV_NAME is inconsistent.
            // This could happen if a non-GUV venv is active.
            // In this case, we should require the user to specify a GUV env name.
            if env_name_arg.is_none() {
                 return Err(GuvError::Anyhow(anyhow::anyhow!(
                    "A virtual environment is active (VIRTUAL_ENV={}), but it does not appear to be a GUV-managed environment or GUV_ENV_NAME is not set/inconsistent.\nPlease specify a GUV environment name or activate a GUV environment.",
                    active_env_path.display()
                )));
            }
        }
    }

    // 2. If no GUV environment is active, or if it was inconsistent and env_name_arg was provided, use env_name_arg
    if let Some(name) = env_name_arg {
        let env_path = ensure_env_exists(name)?;
        return Ok((env_path, name.clone()));
    }

    // 3. If no active GUV env and no name provided, it's an error.
    Err(GuvError::Anyhow(anyhow::anyhow!(
        "No GUV environment is active, and no environment name was specified.\nUse 'guv activate <name>' or provide the environment name to the command."
    )))
}