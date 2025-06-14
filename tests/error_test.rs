mod common;

#[test]
fn test_nonexistent_environment() {
    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    
    // Try to activate a non-existent environment
    let output = common::run_muv_command(
        &["activate", "nonexistent_env"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8 output");
    assert!(stderr.contains("not found") || stderr.contains("does not exist"));
}

#[test]
fn test_invalid_command() {
    let output = common::run_muv_command(&["invalid_command"], &[]);
    assert!(!output.status.success());
}

#[test]
fn test_path_nonexistent_env() {
    let temp_dir = common::setup_test_env();
    let muv_home = temp_dir.path().join(".muv");
    
    let output = common::run_muv_command(
        &["path", "nonexistent_env"], 
        &[("MUV_HOME", muv_home.to_str().unwrap())]
    );
    
    assert!(!output.status.success());
}