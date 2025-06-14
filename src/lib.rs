pub mod cli;
pub mod commands;
pub mod error;
pub mod utils;

// Re-export key components for easier use in tests
pub use error::{MuvError, Result};
