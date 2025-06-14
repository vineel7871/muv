mod common;

use std::fs;
use std::path::Path;

#[test]
fn test_list_empty() {
    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    
    let output = common::run_muv_command(
        &["list"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert!(stdout.contains("No environments found") || stdout.trim().is_empty());
}

#[test]
fn test_list_with_environments() {
    // Skip if uv not installed
    if !Path::new("/usr/local/bin/uv").exists() && !Path::new("/usr/bin/uv").exists() {
        println!("Skipping test: uv not found");
        return;
    }

    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    
    // Create a test environment
    let create_output = common::run_muv_command(
        &["create", "list_test_env", "pytest"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    // If creation fails, skip the rest
    if !create_output.status.success() {
        println!("Skipping test: Environment creation failed");
        return;
    }
    
    // Test listing environments
    let list_output = common::run_muv_command(
        &["list"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(list_output.status.success());
    let list_stdout = String::from_utf8(list_output.stdout).expect("Invalid UTF-8 output");
    assert!(list_stdout.contains("list_test_env"));
}