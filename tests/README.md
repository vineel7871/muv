# Testing Guide for muv

This directory contains tests for the muv project. The tests are organized as follows:

## Test Structure

- `common/mod.rs`: Contains helper functions used across multiple test files
- `cli_test.rs`: Tests for CLI functionality (version, help, etc.)
- `commands_test.rs`: Integration tests for commands (create, delete, etc.)
- `utils_test.rs`: Basic tests for utility functions
- `unit/`: Contains unit tests for individual modules

## Running Tests

Run all tests:
```bash
cargo test
```

Run a specific test:
```bash
cargo test test_cli_version
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Writing New Tests

### Integration Tests

Integration tests should be placed in the `tests/` directory. These tests use the compiled binary and test the functionality from a user's perspective.

Example:
```rust
#[test]
fn test_some_feature() {
    let output = common::run_muv_command(&["some-command", "arg"], &[]);
    assert!(output.status.success());
    // Additional assertions...
}
```

### Unit Tests

Unit tests should be placed in the `tests/unit/` directory or within the source files using `#[cfg(test)]` modules.

Example:
```rust
#[test]
fn test_some_function() {
    let result = some_function();
    assert_eq!(result, expected_value);
}
```

## Test Environment

Tests use the `tempfile` crate to create isolated test environments. The `MUV_HOME` environment variable can be set to override the default location for testing purposes.