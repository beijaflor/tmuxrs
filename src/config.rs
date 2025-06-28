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
        let config_file = config_dir.join(format!("{}.yml", session_name));

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
