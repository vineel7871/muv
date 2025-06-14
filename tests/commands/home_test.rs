mod common;

use std::env;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_home_command() {
    // Test with default home
    let default_output = common::run_muv_command(&["home"], &[]);
    assert!(default_output.status.success());
    let default_home = String::from_utf8(default_output.stdout).expect("Invalid UTF-8 output").trim().to_string();
    assert!(!default_home.is_empty());
    
    // Test with custom home
    let temp_dir = common::setup_test_env();
    let custom_home = temp_dir.path().join(".muv");
    fs::create_dir_all(&custom_home).expect("Failed to create test home directory");
    
    let custom_output = common::run_muv_command(
        &["home"], 
        &[("MUV_HOME", custom_home.to_str().unwrap())]
    );
    
    assert!(custom_output.status.success());
    let output_home = String::from_utf8(custom_output.stdout).expect("Invalid UTF-8 output").trim().to_string();
    assert_eq!(output_home, custom_home.to_str().unwrap());
}