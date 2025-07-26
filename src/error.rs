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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_not_found_error_display() {
        let error = TmuxrsError::ConfigNotFound("~/.config/tmuxrs/test.yml".to_string());
        let display = format!("{error}");
        assert_eq!(
            display,
            "Configuration file not found: ~/.config/tmuxrs/test.yml"
        );
    }

    #[test]
    fn test_tmux_error_display() {
        let error = TmuxrsError::TmuxError("Session already exists".to_string());
        let display = format!("{error}");
        assert_eq!(display, "tmux command failed: Session already exists");
    }

    #[test]
    fn test_yaml_error_conversion() {
        let yaml_str = "invalid: yaml: content: {{";
        let result: Result<serde_yaml::Value> =
            serde_yaml::from_str(yaml_str).map_err(TmuxrsError::from);

        assert!(result.is_err());
        let error = result.unwrap_err();
        let display = format!("{error}");
        assert!(display.contains("Failed to parse YAML:"));
    }

    #[test]
    fn test_io_error_conversion() {
        use std::io;

        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let tmuxrs_error = TmuxrsError::from(io_error);
        let display = format!("{tmuxrs_error}");
        assert!(display.contains("IO error:"));
        assert!(display.contains("File not found"));
    }

    #[test]
    fn test_error_debug_format() {
        let error = TmuxrsError::ConfigNotFound("test.yml".to_string());
        let debug = format!("{error:?}");
        assert!(debug.contains("ConfigNotFound"));
        assert!(debug.contains("test.yml"));
    }
}
