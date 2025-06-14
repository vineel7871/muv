pub mod commands;
pub mod error;
pub mod utils;
pub mod cli;

// Re-export key components for easier use in tests
pub use error::{MuvError, Result};