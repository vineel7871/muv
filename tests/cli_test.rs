mod common;

use std::fs;
use std::path::Path;

#[test]
fn test_cli_version() {
    let output = common::run_muv_command(&["--version"], &[]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert!(stdout.contains("muv"));  // Should contain the name
    assert!(stdout.contains("0.1"));  // Should contain version
}

#[test]
fn test_cli_help() {
    let output = common::run_muv_command(&["--help"], &[]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    
    // Check for expected commands
    assert!(stdout.contains("create"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("activate"));
    assert!(stdout.contains("delete"));
}

#[test]
fn test_home_command() {
    let output = common::run_muv_command(&["home"], &[]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");
    assert!(!stdout.trim().is_empty());
}