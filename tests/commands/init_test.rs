use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use super::common;
use anyhow::Result;

#[test]
fn test_init_creates_config_files() -> Result<()> {
    let temp_dir = common::setup_test_env();
    let home_dir = temp_dir.path().to_path_buf();
    
    // Set up environment for testing
    let shell_config_path = home_dir.join(".bashrc");
    let functions_path = home_dir.join(".muv-functions.sh");
    
    // Create empty shell config file
    File::create(&shell_config_path)?;
    
    // Run the init command with HOME set to our temp directory
    let output = common::run_muv_command(
        &["init"],
        &[
            ("HOME", home_dir.to_str().unwrap()),
            ("SHELL", "/bin/bash"),
        ],
    );
    
    // Check command succeeded
    assert!(output.status.success(), "Init command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify files were created
    assert!(shell_config_path.exists(), "Shell config file was not created");
    assert!(functions_path.exists(), "Functions file was not created");
    
    // Check content of shell config file
    let mut shell_config_content = String::new();
    File::open(&shell_config_path)?.read_to_string(&mut shell_config_content)?;
    
    assert!(shell_config_content.contains("# MUV INIT START"), "Shell config doesn't contain init start marker");
    assert!(shell_config_content.contains("# MUV INIT END"), "Shell config doesn't contain init end marker");
    assert!(shell_config_content.contains("export MUV_BINARY_PATH="), "Shell config doesn't set MUV_BINARY_PATH");
    
    // Check content of functions file
    let mut functions_content = String::new();
    File::open(&functions_path)?.read_to_string(&mut functions_content)?;
    
    assert!(functions_content.contains("muv() {"), "Functions file doesn't contain muv function");
    assert!(functions_content.contains("activate)"), "Functions file doesn't handle activate command");
    assert!(functions_content.contains("deactivate)"), "Functions file doesn't handle deactivate command");
    
    Ok(())
}

#[test]
fn test_init_force_flag_overwrites_existing_config() -> Result<()> {
    let temp_dir = common::setup_test_env();
    let home_dir = temp_dir.path().to_path_buf();
    
    // Set up environment for testing
    let shell_config_path = home_dir.join(".bashrc");
    
    // Create shell config file with existing muv block
    let initial_content = r#"
# Some existing shell config
export PATH=$PATH:/usr/local/bin

# MUV INIT START
# Old muv config
export MUV_BINARY_PATH="/old/path/to/muv"
# MUV INIT END

# More shell config
alias ll='ls -la'
"#;
    
    fs::write(&shell_config_path, initial_content)?;
    
    // Run the init command with --force flag
    let output = common::run_muv_command(
        &["init", "--force"],
        &[
            ("HOME", home_dir.to_str().unwrap()),
            ("SHELL", "/bin/bash"),
        ],
    );
    
    // Check command succeeded
    assert!(output.status.success(), "Init command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check content of shell config file
    let mut shell_config_content = String::new();
    File::open(&shell_config_path)?.read_to_string(&mut shell_config_content)?;
    
    // Verify old block was removed and new one added
    assert!(!shell_config_content.contains("export MUV_BINARY_PATH=\"/old/path/to/muv\""), 
           "Old config was not removed");
    assert!(shell_config_content.contains("# MUV INIT START"), "New init start marker not found");
    assert!(shell_config_content.contains("# MUV INIT END"), "New init end marker not found");
    
    // Verify the non-muv parts of the config were preserved
    assert!(shell_config_content.contains("# Some existing shell config"), 
           "Existing config header was not preserved");
    assert!(shell_config_content.contains("export PATH=$PATH:/usr/local/bin"), 
           "Existing PATH was not preserved");
    assert!(shell_config_content.contains("alias ll='ls -la'"), 
           "Existing alias was not preserved");
    
    Ok(())
}

#[test]
fn test_init_without_force_preserves_existing_config() -> Result<()> {
    let temp_dir = common::setup_test_env();
    let home_dir = temp_dir.path().to_path_buf();
    
    // Set up environment for testing
    let shell_config_path = home_dir.join(".bashrc");
    
    // Create shell config file with existing muv block
    let initial_content = r#"
# Some existing shell config
export PATH=$PATH:/usr/local/bin

# MUV INIT START
# Old muv config
export MUV_BINARY_PATH="/old/path/to/muv"
# MUV INIT END

# More shell config
alias ll='ls -la'
"#;
    
    fs::write(&shell_config_path, initial_content)?;
    
    // Run the init command without --force flag
    let output = common::run_muv_command(
        &["init"],
        &[
            ("HOME", home_dir.to_str().unwrap()),
            ("SHELL", "/bin/bash"),
        ],
    );
    
    // Check command succeeded
    assert!(output.status.success(), "Init command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Check content of shell config file
    let mut shell_config_content = String::new();
    File::open(&shell_config_path)?.read_to_string(&mut shell_config_content)?;
    
    // Verify the original content was preserved
    assert_eq!(shell_config_content, initial_content, "Shell config was modified when it shouldn't have been");
    
    Ok(())
}

#[test]
fn test_init_with_zsh_shell() -> Result<()> {
    let temp_dir = common::setup_test_env();
    let home_dir = temp_dir.path().to_path_buf();
    
    // Set up environment for testing
    let shell_config_path = home_dir.join(".zshrc");
    
    // Run the init command with zsh shell
    let output = common::run_muv_command(
        &["init"],
        &[
            ("HOME", home_dir.to_str().unwrap()),
            ("SHELL", "/bin/zsh"),
        ],
    );
    
    // Check command succeeded
    assert!(output.status.success(), "Init command failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify files were created
    assert!(shell_config_path.exists(), "ZSH config file was not created");
    
    // Check content of shell config file
    let mut shell_config_content = String::new();
    File::open(&shell_config_path)?.read_to_string(&mut shell_config_content)?;
    
    assert!(shell_config_content.contains("# MUV INIT START"), "ZSH config doesn't contain init start marker");
    assert!(shell_config_content.contains("# MUV INIT END"), "ZSH config doesn't contain init end marker");
    
    Ok(())
}

#[test]
fn test_init_with_unsupported_shell() {
    let temp_dir = common::setup_test_env();
    let home_dir = temp_dir.path().to_path_buf();
    
    // Run the init command with an unsupported shell
    let output = common::run_muv_command(
        &["init"],
        &[
            ("HOME", home_dir.to_str().unwrap()),
            ("SHELL", "/bin/fish"),
        ],
    );
    
    // Check command failed
    assert!(!output.status.success(), "Init command succeeded when it should have failed");
    
    // Check error message
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unsupported shell"), "Error message doesn't mention unsupported shell");
    assert!(stderr.contains("fish"), "Error message doesn't mention the shell name");
}