use thiserror::Error;

#[derive(Error, Debug)]
pub enum MuvError {
    #[error("Environment '{0}' already exists.")]
    EnvironmentAlreadyExists(String),
    #[error("Environment '{0}' not found.")]
    EnvironmentNotFound(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("UV command failed: {0}")]
    UvCommandFailed(String),
    #[error("Failed to determine MUV home directory.")]
    HomeDirError,
    #[error("User did not confirm deletion.")]
    DeletionNotConfirmed,
    #[error("Failed to parse pyproject.toml: {0}")]
    TomlParseError(#[from] toml::de::Error),
    #[error("Failed to serialize pyproject.toml: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("'{0}' is not installed or not in PATH.")]
    UvNotInstalled(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, MuvError>;
