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
#[serde(untagged)]
pub enum PaneConfig {
    Multiple(Vec<String>),
    Named(std::collections::HashMap<String, serde_yaml::Value>),
    Simple(String),
    Null,
}

impl PaneConfig {
    /// Extract commands from this pane configuration
    pub fn commands(&self) -> Vec<String> {
        match self {
            PaneConfig::Simple(cmd) => {
                if cmd.trim().is_empty() {
                    vec![]
                } else {
                    vec![cmd.clone()]
                }
            }
            PaneConfig::Multiple(cmds) => cmds.clone(),
            PaneConfig::Named(map) => {
                // Extract commands from the named pane
                if let Some((_, value)) = map.iter().next() {
                    match value {
                        serde_yaml::Value::String(cmd) => {
                            if cmd.trim().is_empty() {
                                vec![]
                            } else {
                                vec![cmd.clone()]
                            }
                        }
                        serde_yaml::Value::Sequence(seq) => {
                            seq.iter()
                                .filter_map(|v| {
                                    if let serde_yaml::Value::String(s) = v {
                                        Some(s.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect()
                        }
                        _ => vec![],
                    }
                } else {
                    vec![]
                }
            }
            PaneConfig::Null => vec![],
        }
    }

    /// Check if this pane configuration should not execute any commands
    pub fn is_empty(&self) -> bool {
        self.commands().is_empty()
    }

    /// Get the name of a named pane, if any
    pub fn name(&self) -> Option<String> {
        match self {
            PaneConfig::Named(map) => {
                map.keys().next().map(|k| k.clone())
            }
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WindowLayout {
    pub layout: Option<String>,
    pub panes: Vec<PaneConfig>,
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

    // TDD: Failing tests for enhanced pane configuration
    #[test]
    fn test_parse_empty_pane_as_empty_string() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - vim
  - ""
  - top
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 3);
        
        // Second pane should be empty string
        match &layout_config.panes[1] {
            PaneConfig::Simple(cmd) if cmd.is_empty() => {}, // Expected
            _ => panic!("Expected empty string pane config for empty string"),
        }
    }

    #[test]
    fn test_parse_empty_pane_as_null() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - vim
  - ~
  - top
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 3);
        
        // Second pane should be null
        match &layout_config.panes[1] {
            PaneConfig::Null => {}, // Expected  
            _ => panic!("Expected null pane config for null value"),
        }
    }

    #[test]
    fn test_parse_named_pane_single_command() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - editor: vim
  - console: bash
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 2);
        
        // First pane should be named "editor" with command "vim"
        match &layout_config.panes[0] {
            PaneConfig::Named(map) => {
                assert_eq!(map.len(), 1);
                let (name, value) = map.iter().next().unwrap();
                assert_eq!(name, "editor");
                if let serde_yaml::Value::String(cmd) = value {
                    assert_eq!(cmd, "vim");
                } else {
                    panic!("Expected string command");
                }
            },
            _ => panic!("Expected named pane config"),
        }
    }

    #[test]
    fn test_parse_named_pane_empty_command() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - editor: vim
  - console: ""
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 2);
        
        // Second pane should be named "console" with empty command
        match &layout_config.panes[1] {
            PaneConfig::Named(map) => {
                assert_eq!(map.len(), 1);
                let (name, value) = map.iter().next().unwrap();
                assert_eq!(name, "console");
                if let serde_yaml::Value::String(cmd) = value {
                    assert_eq!(cmd, "");
                } else {
                    panic!("Expected string command");
                }
            },
            _ => panic!("Expected named pane config with empty command"),
        }
    }

    #[test]
    fn test_parse_multiple_commands_array() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - vim
  - [cd frontend, npm start]
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 2);
        
        // Second pane should be multiple commands
        match &layout_config.panes[1] {
            PaneConfig::Multiple(commands) => {
                assert_eq!(commands.len(), 2);
                assert_eq!(commands[0], "cd frontend");
                assert_eq!(commands[1], "npm start");
            },
            _ => panic!("Expected multiple commands pane config"),
        }
    }

    #[test]
    fn test_parse_named_pane_multiple_commands() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - editor: vim
  - server: [cd backend, rails server]
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 2);
        
        // Second pane should be named with multiple commands
        match &layout_config.panes[1] {
            PaneConfig::Named(map) => {
                assert_eq!(map.len(), 1);
                let (name, value) = map.iter().next().unwrap();
                assert_eq!(name, "server");
                if let serde_yaml::Value::Sequence(commands) = value {
                    assert_eq!(commands.len(), 2);
                    if let (serde_yaml::Value::String(cmd1), serde_yaml::Value::String(cmd2)) = 
                       (&commands[0], &commands[1]) {
                        assert_eq!(cmd1, "cd backend");
                        assert_eq!(cmd2, "rails server");
                    } else {
                        panic!("Expected string commands");
                    }
                } else {
                    panic!("Expected sequence of commands");
                }
            },
            _ => panic!("Expected named multiple commands pane config"),
        }
    }

    #[test]
    fn test_backward_compatibility_simple_strings() {
        let yaml_content = r#"
layout: main-vertical
panes:
  - vim
  - npm start
  - top
"#;
        
        let layout_config: WindowLayout = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(layout_config.panes.len(), 3);
        
        // All panes should parse as simple commands (backward compatibility)
        match &layout_config.panes[0] {
            PaneConfig::Simple(cmd) => assert_eq!(cmd, "vim"),
            _ => panic!("Expected simple pane config for backward compatibility"),
        }
        
        match &layout_config.panes[1] {
            PaneConfig::Simple(cmd) => assert_eq!(cmd, "npm start"),
            _ => panic!("Expected simple pane config for backward compatibility"),
        }
        
        match &layout_config.panes[2] {
            PaneConfig::Simple(cmd) => assert_eq!(cmd, "top"),
            _ => panic!("Expected simple pane config for backward compatibility"),
        }
    }

    // TDD: Tests for PaneConfig helper methods
    #[test]
    fn test_pane_config_commands_simple() {
        let pane = PaneConfig::Simple("vim".to_string());
        assert_eq!(pane.commands(), vec!["vim"]);

        let empty_pane = PaneConfig::Simple("".to_string());
        assert_eq!(empty_pane.commands(), Vec::<String>::new());
    }

    #[test]
    fn test_pane_config_commands_multiple() {
        let pane = PaneConfig::Multiple(vec!["cd frontend".to_string(), "npm start".to_string()]);
        assert_eq!(pane.commands(), vec!["cd frontend", "npm start"]);
    }

    #[test]
    fn test_pane_config_commands_null() {
        let pane = PaneConfig::Null;
        assert_eq!(pane.commands(), Vec::<String>::new());
    }

    #[test]
    fn test_pane_config_commands_named_single() {
        let mut map = std::collections::HashMap::new();
        map.insert("editor".to_string(), serde_yaml::Value::String("vim".to_string()));
        let pane = PaneConfig::Named(map);
        
        assert_eq!(pane.commands(), vec!["vim"]);
        assert_eq!(pane.name(), Some("editor".to_string()));
    }

    #[test] 
    fn test_pane_config_commands_named_multiple() {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "server".to_string(), 
            serde_yaml::Value::Sequence(vec![
                serde_yaml::Value::String("cd backend".to_string()),
                serde_yaml::Value::String("rails server".to_string()),
            ])
        );
        let pane = PaneConfig::Named(map);
        
        assert_eq!(pane.commands(), vec!["cd backend", "rails server"]);
        assert_eq!(pane.name(), Some("server".to_string()));
    }

    #[test]
    fn test_pane_config_is_empty() {
        assert!(PaneConfig::Simple("".to_string()).is_empty());
        assert!(PaneConfig::Null.is_empty());
        assert!(!PaneConfig::Simple("vim".to_string()).is_empty());
        assert!(!PaneConfig::Multiple(vec!["test".to_string()]).is_empty());
    }
}
