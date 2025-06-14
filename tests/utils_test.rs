mod common;

use std::fs;
use std::path::PathBuf;

#[test]
fn test_muv_home_directory() {
    let output = common::run_muv_command(&["home"], &[]);
    assert!(output.status.success());

    let home_path = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output")
        .trim()
        .to_string();
    assert!(!home_path.is_empty());

    let path = PathBuf::from(&home_path);
    assert!(path.exists() || !path.is_absolute());
}

#[test]
fn test_list_environments_empty() {
    // Set a custom MUV_HOME for this test
    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");

    let output = common::run_muv_command(
        &["list"],
        &[("MUV_HOME", temp_dir.path().join(".muv").to_str().unwrap())],
    );

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert!(stdout.contains("No environments found") || stdout.trim().is_empty());
}
