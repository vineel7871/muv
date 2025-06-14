use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

// Helper function to create a temporary directory for tests
pub fn setup_test_env() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

// Helper to run the muv binary with arguments
pub fn run_muv_command(args: &[&str], env_vars: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_muv"));
    cmd.args(args);

    for (key, val) in env_vars {
        cmd.env(key, val);
    }

    cmd.output().expect("Failed to execute muv command")
}

// Helper to get a path in the temp directory
pub fn temp_path(temp_dir: &TempDir, path: &str) -> PathBuf {
    temp_dir.path().join(path)
}
