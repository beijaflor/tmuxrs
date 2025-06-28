use thiserror::Error;

pub type Result<T> = std::result::Result<T, TmuxrsError>;

#[derive(Debug, Error)]
pub enum TmuxrsError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),

    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("tmux command failed: {0}")]
    #[allow(dead_code)]
    TmuxError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
