use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbJumpError {
    #[error("Configuration file not found at {0}")]
    ConfigNotFound(String),

    #[error("Failed to parse configuration: {0}")]
    ConfigParseError(String),

    #[error("Database alias '{0}' not found")]
    AliasNotFound(String),

    #[error("Duplicate alias '{0}' found in configuration")]
    DuplicateAlias(String),

    #[error("Invalid alias '{0}': must contain only letters, numbers, hyphens, and underscores")]
    InvalidAliasFormat(String),

    #[error("Invalid port number: {0}")]
    InvalidPort(u16),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("CLI tool '{0}' not found in PATH. Please install it first.")]
    CliToolNotFound(String),

    #[error("Failed to execute command: {0}")]
    ExecutionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, DbJumpError>;
