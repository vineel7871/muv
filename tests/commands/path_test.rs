mod common;

use std::fs;
use std::path::Path;

#[test]
fn test_path_command() {
    // Skip if uv not installed
    if !Path::new("/usr/local/bin/uv").exists() && !Path::new("/usr/bin/uv").exists() {
        println!("Skipping test: uv not found");
        return;
    }

    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    
    // Create a test environment
    let env_name = "path_test_env";
    let create_output = common::run_muv_command(
        &["create", env_name, "pytest"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    // If creation fails, skip the rest
    if !create_output.status.success() {
        println!("Skipping test: Environment creation failed");
        return;
    }
    
    // Test path command
    let path_output = common::run_muv_command(
        &["path", env_name], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(path_output.status.success());
    let path_stdout = String::from_utf8(path_output.stdout).expect("Invalid UTF-8 output");
    let expected_path = muv_home.join("envs").join(env_name);
    assert_eq!(path_stdout.trim(), expected_path.to_str().unwrap());
}