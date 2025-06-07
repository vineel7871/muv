// src/utils.rs
use crate::error::{GuvError, Result}; // Or use anyhow::Result
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;

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