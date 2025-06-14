use crate::error::{MuvError, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const ACTIVE_ENV_VAR: &str = "VIRTUAL_ENV";
pub const MUV_ACTIVE_ENV_NAME_VAR: &str = "MUV_ENV_NAME";

pub fn get_muv_home() -> Result<PathBuf> {
    // Check if MUV_HOME environment variable is set (useful for testing)
    if let Ok(muv_home) = std::env::var("MUV_HOME") {
        return Ok(PathBuf::from(muv_home));
    }

    let base_dir = dirs::data_dir()
        .or_else(dirs::home_dir)
        .ok_or(MuvError::HomeDirError)?;
    Ok(base_dir.join(".muv"))
}

pub fn get_envs_dir() -> Result<PathBuf> {
    let path = get_muv_home()?.join("envs");
    if !path.exists() {
        fs::create_dir_all(&path).map_err(MuvError::IoError)?;
    }
    Ok(path)
}

pub fn get_env_path(name: &str) -> Result<PathBuf> {
    Ok(get_envs_dir()?.join(name))
}

pub fn ensure_env_exists(name: &str) -> Result<PathBuf> {
    let path = get_env_path(name)?;
    if !path.exists() || !path.join("pyvenv.cfg").exists() {
        return Err(MuvError::EnvironmentNotFound(name.to_string()));
    }
    Ok(path)
}

pub fn check_uv_exists() -> Result<()> {
    Command::new("uv")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| MuvError::UvCommandFailed(format!("Failed to execute uv: {}", e)))
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(MuvError::UvNotInstalled("uv".to_string()))
            }
        })
}

pub fn run_uv_command(
    args: &[&str],
    current_dir: Option<&Path>,
    env_vars: Vec<(&str, &Path)>,
) -> Result<()> {
    let mut cmd = Command::new("uv");
    cmd.args(args);
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
    for (key, val) in env_vars {
        cmd.env(key, val);
    }

    // dbg!(&cmd);
    let status = cmd
        .status()
        .map_err(|e| MuvError::UvCommandFailed(format!("Failed to execute uv: {}", e)))?;
    // dbg!(&status);
    if !status.success() {
        let err_msg = format!("uv {} failed with status: {}", args.join(" "), status);
        return Err(MuvError::UvCommandFailed(err_msg));
    }
    Ok(())
}

pub fn get_command_output(
    program: &str,
    args: &[&str],
    current_dir: Option<&Path>,
    env_vars: Vec<(&str, &Path)>,
) -> Result<String> {
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
        .map_err(|e| MuvError::UvCommandFailed(format!("Failed to execute {}: {}", program, e)))?;

    if !output.status.success() {
        let err_msg = format!(
            "{} {} failed with status: {}. Stderr: {}",
            program,
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(MuvError::UvCommandFailed(err_msg));
    }
    String::from_utf8(output.stdout)
        .map_err(|e| MuvError::UvCommandFailed(format!("Failed to parse output as UTF-8: {}", e)))
}

pub fn _create_basic_pyproject_toml(project_path: &Path) -> Result<()> {
    let toml_content = r#"[project]
name = "muv-environment"
version = "0.1.0"
description = "A Python environment managed by MUV."
requires-python = ">=3.8"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#;
    let toml_file_path = project_path.join("pyproject.toml");
    fs::write(toml_file_path, toml_content).map_err(MuvError::IoError)
}

pub fn get_active_or_specified_env(env_name_arg: Option<&String>) -> Result<(PathBuf, String)> {
    if let (Ok(active_env_path_str), Ok(active_muv_name)) =
        (env::var(ACTIVE_ENV_VAR), env::var(MUV_ACTIVE_ENV_NAME_VAR))
    {
        let active_env_path = PathBuf::from(active_env_path_str);
        let envs_dir = get_envs_dir()?;
        if active_env_path.starts_with(&envs_dir)
            && active_env_path.file_name().and_then(|s| s.to_str()) == Some(&active_muv_name)
            && active_env_path.join("pyvenv.cfg").exists()
        {
            if let Some(name_arg) = env_name_arg {
                if name_arg != &active_muv_name {
                    return Err(MuvError::Anyhow(anyhow::anyhow!(
                        "An environment ('{}') is already active, but you specified a different one ('{}').\nDeactivate the current environment or omit the environment name argument.",
                        active_muv_name,
                        name_arg
                    )));
                }
            }
            println!("Using active MUV environment: {}", active_muv_name);
            return Ok((active_env_path, active_muv_name));
        } else {
            if env_name_arg.is_none() {
                return Err(MuvError::Anyhow(anyhow::anyhow!(
                    "A virtual environment is active (VIRTUAL_ENV={}), but it does not appear to be a MUV-managed environment or MUV_ENV_NAME is not set/inconsistent.\nPlease specify a MUV environment name or activate a MUV environment.",
                    active_env_path.display()
                )));
            }
        }
    }

    if let Some(name) = env_name_arg {
        let env_path = ensure_env_exists(name)?;
        return Ok((env_path, name.clone()));
    }

    Err(MuvError::Anyhow(anyhow::anyhow!(
        "No MUV environment is active, and no environment name was specified.\nUse 'muv activate <n>' or provide the environment name to the command."
    )))
}
