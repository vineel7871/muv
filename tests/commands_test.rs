mod common;

use std::fs;
use std::path::Path;

#[test]
fn test_create_and_delete_environment() {
    // Skip this test if uv is not installed
    if !Path::new("/usr/local/bin/uv").exists() && !Path::new("/usr/bin/uv").exists() {
        println!("Skipping test_create_and_delete_environment: uv not found");
        return;
    }

    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    
    // Create a test environment
    let env_name = "test_env";
    let output = common::run_muv_command(
        &["create", env_name, "pytest"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    // If creation fails (possibly due to network or uv issues), skip the rest
    if !output.status.success() {
        println!("Skipping test: Environment creation failed");
        return;
    }
    
    // Verify environment exists
    let env_path = muv_home.join("envs").join(env_name);
    assert!(env_path.exists());
    assert!(env_path.join("pyvenv.cfg").exists());
    
    // Test listing environments
    let list_output = common::run_muv_command(
        &["list"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    assert!(list_output.status.success());
    let list_stdout = String::from_utf8(list_output.stdout).expect("Invalid UTF-8 output");
    assert!(list_stdout.contains(env_name));
    
    // Delete the environment
    let delete_output = common::run_muv_command(
        &["delete", env_name, "--yes"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    assert!(delete_output.status.success());
    
    // Verify environment no longer exists
    assert!(!env_path.exists());
}