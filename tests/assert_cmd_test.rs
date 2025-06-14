use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_env() -> TempDir {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let muv_home = temp_dir.path().join(".muv");
    fs::create_dir_all(muv_home.join("envs")).expect("Failed to create test envs directory");
    temp_dir
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("muv").expect("Binary not found");
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("muv").expect("Binary not found");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("muv"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("Options:"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("muv").expect("Binary not found");
    cmd.arg("invalid-command");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}

#[test]
fn test_home_command() {
    let temp_dir = setup_test_env();
    let muv_home = temp_dir.path().join(".muv");

    let mut cmd = Command::cargo_bin("muv").expect("Binary not found");
    cmd.arg("home").env("MUV_HOME", muv_home.to_str().unwrap());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(muv_home.to_str().unwrap()));
}
