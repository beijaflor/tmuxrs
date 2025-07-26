use crate::error::{Result, TmuxrsError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub root: Option<String>,
    pub windows: Vec<WindowConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WindowConfig {
    Simple(String),
    Complex {
        #[serde(flatten)]
        window: HashMap<String, String>,
    },
    WithLayout {
        #[serde(flatten)]
        window: HashMap<String, WindowLayout>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowLayout {
    pub layout: Option<String>,
    pub panes: Vec<String>,
}

impl Config {
    /// Detect session name from directory basename
    /// If path is None, uses current directory
    #[allow(dead_code)]
    pub fn detect_session_name(path: Option<&Path>) -> Result<String> {
        let dir = match path {
            Some(p) => p.to_path_buf(),
            None => std::env::current_dir()?,
        };
        let basename = dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                TmuxrsError::ConfigNotFound("Could not determine directory name".to_string())
            })?;
        Ok(basename.to_string())
    }

    /// Get config file path for a session name
    #[allow(dead_code)]
    pub fn get_config_file_path(session_name: &str) -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            TmuxrsError::ConfigNotFound("Could not find home directory".to_string())
        })?;

        let config_dir = home_dir.join(".config").join("tmuxrs");
        let config_file = config_dir.join(format!("{session_name}.yml"));

        Ok(config_file)
    }

    /// Load configuration for a session
    #[allow(dead_code)]
    pub fn load(session_name: &str) -> Result<Config> {
        let config_path = Self::get_config_file_path(session_name)?;

        if !config_path.exists() {
            return Err(TmuxrsError::ConfigNotFound(format!(
                "Configuration file not found: {}",
                config_path.display()
            )));
        }

        Self::parse_file(&config_path)
    }

    /// Parse configuration from a YAML file
    #[allow(dead_code)]
    pub fn parse_file(file_path: &Path) -> Result<Config> {
        let content = std::fs::read_to_string(file_path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_detect_session_name_from_directory() {
        // Test directory basename detection
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("my-awesome-project");
        std::fs::create_dir(&project_path).unwrap();

        let session_name = Config::detect_session_name(Some(&project_path)).unwrap();
        assert_eq!(session_name, "my-awesome-project");
    }

    #[test]
    fn test_get_config_file_path() {
        let config_path = Config::get_config_file_path("test-session").unwrap();

        // Should be ~/.config/tmuxrs/test-session.yml
        assert!(config_path.to_string_lossy().contains(".config/tmuxrs"));
        assert!(config_path.to_string_lossy().ends_with("test-session.yml"));
    }

    #[test]
    fn test_load_config_file_not_found() {
        let result = Config::load("nonexistent-session");

        match result {
            Err(TmuxrsError::ConfigNotFound(_)) => {
                // Expected error
            }
            _ => panic!("Expected ConfigNotFound error"),
        }
    }

    #[test]
    fn test_parse_yaml_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test.yml");

        let yaml_content = r#"
name: test-session
root: ~/projects/test
windows:
  - editor: vim
  - server: rails server
"#;

        std::fs::write(&config_file, yaml_content).unwrap();

        let config = Config::parse_file(&config_file).unwrap();
        assert_eq!(config.name, "test-session");
        assert_eq!(config.root, Some("~/projects/test".to_string()));
        assert_eq!(config.windows.len(), 2);
    }

    #[test]
    fn test_configuration_discovery_integration() {
        // This test verifies the complete configuration discovery flow:
        // 1. Pass a directory path
        // 2. Detect session name from directory
        // 3. Resolve config path
        // 4. Load config if it exists

        let temp_dir = TempDir::new().unwrap();

        // Create a project directory
        let project_dir = temp_dir.path().join("my-rust-project");
        std::fs::create_dir(&project_dir).unwrap();

        // Create a mock config directory structure
        let config_dir = temp_dir.path().join(".config").join("tmuxrs");
        std::fs::create_dir_all(&config_dir).unwrap();

        // Create config file for the project
        let config_file = config_dir.join("my-rust-project.yml");
        let yaml_content = r#"
name: my-rust-project
root: ~/code/my-rust-project
windows:
  - editor: vim src/main.rs
  - server: cargo run
  - git: lazygit
"#;
        std::fs::write(&config_file, yaml_content).unwrap();

        // Test the discovery flow
        let detected_name = Config::detect_session_name(Some(&project_dir)).unwrap();
        assert_eq!(detected_name, "my-rust-project");

        // In real usage, we'd use dirs::home_dir(), but for testing we'll parse directly
        let loaded_config = Config::parse_file(&config_file).unwrap();
        assert_eq!(loaded_config.name, "my-rust-project");
        assert_eq!(loaded_config.windows.len(), 3);
    }

    #[test]
    fn test_detect_session_name_different_directories() {
        let temp_dir = TempDir::new().unwrap();

        // Test various directory names
        let test_cases = vec![
            "web-app",
            "my-project",
            "tmuxrs",
            "backend-api",
            "123-numbers",
        ];

        for dir_name in test_cases {
            let test_dir = temp_dir.path().join(dir_name);
            std::fs::create_dir(&test_dir).unwrap();

            let detected = Config::detect_session_name(Some(&test_dir)).unwrap();
            assert_eq!(
                detected, dir_name,
                "Failed to detect session name for directory: {dir_name}"
            );
        }
    }

    #[test]
    fn test_detect_session_name_current_directory() {
        // Test that passing None uses current directory
        let current_dir = std::env::current_dir().unwrap();
        let expected_name = current_dir.file_name().unwrap().to_str().unwrap();

        let detected = Config::detect_session_name(None).unwrap();
        assert_eq!(detected, expected_name);
    }
}
