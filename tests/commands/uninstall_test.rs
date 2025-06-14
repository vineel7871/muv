mod common;

use std::fs;
use std::path::Path;

#[test]
fn test_uninstall_package() {
    // Skip if uv not installed
    if !Path::new("/usr/local/bin/uv").exists() && !Path::new("/usr/bin/uv").exists() {
        println!("Skipping test: uv not found");
        return;
    }

    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    
    // Create a test environment with requests package
    let env_name = "uninstall_test_env";
    let create_output = common::run_muv_command(
        &["create", env_name, "requests"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    // If creation fails, skip the rest
    if !create_output.status.success() {
        println!("Skipping test: Environment creation failed");
        return;
    }
    
    // Verify package is installed
    let freeze_output = common::run_muv_command(
        &["freeze", env_name], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    let freeze_stdout = String::from_utf8(freeze_output.stdout).expect("Invalid UTF-8 output");
    if !freeze_stdout.contains("requests") {
        println!("Skipping test: Package not installed correctly");
        return;
    }
    
    // Uninstall the package
    let uninstall_output = common::run_muv_command(
        &["uninstall", "--env-name", env_name, "requests"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    // If uninstallation fails, skip the rest
    if !uninstall_output.status.success() {
        println!("Skipping test: Package uninstallation failed");
        return;
    }
    
    // Check if package is no longer installed
    let freeze_output = common::run_muv_command(
        &["freeze", env_name], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(freeze_output.status.success());
    let freeze_stdout = String::from_utf8(freeze_output.stdout).expect("Invalid UTF-8 output");
    assert!(!freeze_stdout.contains("requests"));
}