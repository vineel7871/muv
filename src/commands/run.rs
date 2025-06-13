use crate::RunArgs;
use crate::utils;
use anyhow::{Context, Result};
use std::process::Command;

pub fn handle_run(args: RunArgs) -> Result<()> {
    let env_path = utils::ensure_env_exists(&args.env_name)?;
    let python_exe = env_path.join("bin").join("python"); // Or Scripts\python.exe on Windows

    if !python_exe.exists() {
        anyhow::bail!(
            "Python interpreter not found at {}. Environment might be corrupted.",
            python_exe.display()
        );
    }

    let (command_to_run, command_args) = args
        .command_and_args
        .split_first()
        .ok_or_else(|| anyhow::anyhow!("No command provided to run"))?;

    let executable_to_run: std::path::PathBuf;
    let final_args: &[String];

    // If the command is "python", use the venv's python. Otherwise, assume it's a script/command in PATH or relative.
    // For robust behavior, we should resolve commands against the venv's bin directory first.
    let first_command_in_venv_bin = env_path.join("bin").join(command_to_run);

    if command_to_run.to_lowercase() == "python" {
        executable_to_run = python_exe.clone();
        final_args = command_args;
    } else if first_command_in_venv_bin.exists() && first_command_in_venv_bin.is_file() {
        // If 'my_script' is in venv/bin, run venv/bin/my_script
        executable_to_run = first_command_in_venv_bin;
        final_args = command_args;
    } else {
        // Run the command as is, relying on it being in PATH or a full path itself
        // This means we are NOT prepending the venv's python path to the system PATH
        // for this command. If the user wants that behavior, they should activate first.
        // This `run` command primarily targets running Python scripts with the venv's interpreter
        // or commands installed *into* the venv's bin.
        // A more advanced `run` might manipulate the PATH environment variable for the subprocess.
        executable_to_run = command_to_run.into();
        final_args = command_args;
    }

    println!(
        "Running in environment '{}': {} {}",
        args.env_name,
        executable_to_run.display(),
        final_args.join(" ")
    );

    let mut cmd = Command::new(&executable_to_run);
    cmd.args(final_args);
    // Crucial: Set VIRTUAL_ENV so subprocesses (like pip if the script calls it) know about the venv
    cmd.env("VIRTUAL_ENV", &env_path);
    // For full activation behavior, one would also need to prepend env_path/bin to PATH
    // let current_path = std::env::var("PATH").unwrap_or_default();
    // let new_path = format!("{}:{}" , env_path.join("bin").display(), current_path);
    // cmd.env("PATH", new_path);

    let status = cmd.status().with_context(|| {
        format!(
            "Failed to execute command: '{} {}'",
            executable_to_run.display(),
            final_args.join(" ")
        )
    })?;

    if !status.success() {
        anyhow::bail!(
            "Command '{} {}' failed with status: {}",
            executable_to_run.display(),
            final_args.join(" "),
            status
        );
    }

    Ok(())
}
