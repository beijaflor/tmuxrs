use crate::config::Config;
use crate::error::{Result, TmuxrsError};
use crate::tmux::TmuxCommand;
use std::path::{Path, PathBuf};

/// Session manager for tmuxrs
#[derive(Default)]
pub struct SessionManager;

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self
    }

    /// Expand tilde (~) and environment variables in paths using shellexpand
    fn expand_path(path: &str) -> Result<PathBuf> {
        // Try full expansion first (handles both tilde and environment variables)
        match shellexpand::full(path) {
            Ok(expanded) => Ok(PathBuf::from(expanded.as_ref())),
            Err(_) => {
                // Fallback: try basic tilde expansion only
                let expanded = shellexpand::tilde(path);
                Ok(PathBuf::from(expanded.as_ref()))
            }
        }
    }

    /// Start a session with optional explicit name
    pub fn start_session(&self, name: Option<&str>, config_dir: Option<&Path>) -> Result<String> {
        // Use default behavior: attach=true, append=false
        self.start_session_with_options(name, config_dir, true, false)
    }

    /// Start a session with full options control
    pub fn start_session_with_options(
        &self,
        name: Option<&str>,
        config_dir: Option<&Path>,
        attach: bool,
        append: bool,
    ) -> Result<String> {
        let session_name = match name {
            Some(n) => n.to_string(),
            None => Config::detect_session_name(None)?,
        };

        // Check if session already exists
        if TmuxCommand::session_exists(&session_name)? {
            if append {
                // TODO: Implement append functionality in Phase 2
                return Err(TmuxrsError::TmuxError(
                    "Append functionality not yet implemented".to_string(),
                ));
            } else if attach {
                // Attach to existing session
                match TmuxCommand::attach_session(&session_name) {
                    Ok(()) => {
                        // This line should never be reached in practice because
                        // successful attach takes over the terminal process
                        return Ok(format!("Attached to existing session '{session_name}'"));
                    }
                    Err(err) => {
                        // Attach failed - could be no TTY, session doesn't exist, etc.
                        return Err(TmuxrsError::TmuxError(format!(
                            "Failed to attach to session '{session_name}': {err}"
                        )));
                    }
                }
            } else {
                return Ok(format!("Session '{session_name}' already exists"));
            }
        }

        // Load configuration
        let config = if let Some(config_dir) = config_dir {
            // Load from custom config directory
            let config_file = config_dir.join(format!("{session_name}.yml"));
            Config::parse_file(&config_file)?
        } else {
            Config::load(&session_name)?
        };

        // Create session
        let root_dir = config.root.as_deref().unwrap_or("~");
        let root_path = Self::expand_path(root_dir)?;
        TmuxCommand::new_session(&session_name, &root_path)?;

        // Create windows
        for (index, window_config) in config.windows.iter().enumerate() {
            match window_config {
                crate::config::WindowConfig::Simple(command) => {
                    let window_name = format!("window-{}", index + 1);
                    // Create window without command to allow proper shell initialization
                    TmuxCommand::new_window(
                        &session_name,
                        &window_name,
                        None, // No command - let shell initialize properly
                        Some(&root_path),
                    )?;
                    // Send command after window is created
                    if !command.trim().is_empty() {
                        TmuxCommand::send_keys(&session_name, &window_name, command)?;
                    }
                }
                crate::config::WindowConfig::Complex { window } => {
                    for (window_name, command) in window {
                        // Create window without command to allow proper shell initialization
                        TmuxCommand::new_window(
                            &session_name,
                            window_name,
                            None, // No command - let shell initialize properly
                            Some(&root_path),
                        )?;
                        // Send command after window is created
                        if !command.trim().is_empty() {
                            TmuxCommand::send_keys(&session_name, window_name, command)?;
                        }
                    }
                }
                crate::config::WindowConfig::WithLayout { window } => {
                    for (window_name, layout_config) in window {
                        // Create the window without command to allow proper shell initialization
                        TmuxCommand::new_window(
                            &session_name,
                            window_name,
                            None, // No command - let shell initialize properly
                            Some(&root_path),
                        )?;

                        // Send first pane command if not empty
                        let first_pane = layout_config.panes.first().ok_or_else(|| {
                            TmuxrsError::TmuxError(
                                "Window layout must have at least one pane".to_string(),
                            )
                        })?;
                        if !first_pane.trim().is_empty() {
                            TmuxCommand::send_keys(&session_name, window_name, first_pane)?;
                        }

                        // Add additional panes by splitting
                        for (pane_index, pane_command) in
                            layout_config.panes.iter().skip(1).enumerate()
                        {
                            // Create split without command to allow proper shell initialization
                            TmuxCommand::split_window_horizontal(
                                &session_name,
                                window_name,
                                "", // Empty command - shell will initialize properly
                                Some(&root_path),
                            )?;
                            // Send command to the new pane after it's created
                            // Pane indices start at 0, first pane is 0, second is 1, etc.
                            let target_pane_index = pane_index + 1; // +1 because we skipped the first pane
                            if !pane_command.trim().is_empty() {
                                TmuxCommand::send_keys_to_pane(
                                    &session_name,
                                    window_name,
                                    target_pane_index,
                                    pane_command,
                                )?;
                            }
                        }

                        // Apply layout if specified
                        if let Some(layout) = &layout_config.layout {
                            TmuxCommand::select_layout(&session_name, window_name, layout)?;
                        }
                    }
                }
            }
        }

        // Handle attachment
        if attach {
            match TmuxCommand::attach_session(&session_name) {
                Ok(()) => {
                    // This line should never be reached in practice because
                    // successful attach takes over the terminal process
                    Ok(format!("Started and attached to session '{session_name}'"))
                }
                Err(err) => {
                    // Attach failed - provide helpful error message
                    Err(TmuxrsError::TmuxError(format!(
                        "Started session '{session_name}' but failed to attach: {err}"
                    )))
                }
            }
        } else {
            Ok(format!("Started detached session '{session_name}'"))
        }
    }

    /// Start a session detecting name from directory
    #[allow(dead_code)]
    pub fn start_session_from_directory(
        &self,
        directory: &Path,
        config_dir: Option<&Path>,
    ) -> Result<String> {
        let session_name = Config::detect_session_name(Some(directory))?;
        self.start_session(Some(&session_name), config_dir)
    }

    /// List available configurations
    pub fn list_configs(&self, config_dir: Option<&Path>) -> Result<Vec<Config>> {
        let search_dir = match config_dir {
            Some(dir) => dir.to_path_buf(),
            None => {
                let home_dir = dirs::home_dir().ok_or_else(|| {
                    TmuxrsError::ConfigNotFound("Could not find home directory".to_string())
                })?;
                home_dir.join(".config").join("tmuxrs")
            }
        };

        if !search_dir.exists() {
            return Ok(Vec::new());
        }

        let mut configs = Vec::new();

        for entry in std::fs::read_dir(&search_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext == "yml" || ext == "yaml")
            {
                match Config::parse_file(&path) {
                    Ok(config) => configs.push(config),
                    Err(_) => continue, // Skip invalid config files
                }
            }
        }

        Ok(configs)
    }

    /// Stop a session
    pub fn stop_session(&self, name: &str) -> Result<String> {
        // Check if session exists first
        if !TmuxCommand::session_exists(name)? {
            return Err(TmuxrsError::TmuxError(format!(
                "Session '{name}' does not exist"
            )));
        }

        TmuxCommand::kill_session(name)?;
        Ok(format!("Stopped session '{name}'"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[test]
    fn test_expand_path_home_directory() {
        // Test tilde expansion
        let path = SessionManager::expand_path("~/projects").unwrap();
        assert!(path.is_absolute());
        assert!(!path.to_string_lossy().contains('~'));
    }

    #[test]
    fn test_expand_path_environment_variable() {
        // Set a test environment variable
        env::set_var("TEST_PATH", "/tmp/test");

        let path = SessionManager::expand_path("$TEST_PATH/project").unwrap();
        assert_eq!(path.to_string_lossy(), "/tmp/test/project");

        // Clean up
        env::remove_var("TEST_PATH");
    }

    #[test]
    fn test_expand_path_no_expansion_needed() {
        // Test absolute path
        let path = SessionManager::expand_path("/usr/local/bin").unwrap();
        assert_eq!(path.to_string_lossy(), "/usr/local/bin");
    }

    #[test]
    fn test_expand_path_combined() {
        // Test combined tilde and env var
        env::set_var("TEST_DIR", "mydir");

        let path = SessionManager::expand_path("~/$TEST_DIR/project").unwrap();
        assert!(path.is_absolute());
        assert!(path.to_string_lossy().contains("mydir/project"));
        assert!(!path.to_string_lossy().contains('~'));
        assert!(!path.to_string_lossy().contains("$TEST_DIR"));

        // Clean up
        env::remove_var("TEST_DIR");
    }

    #[test]
    fn test_list_configs_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SessionManager::new();

        let configs = manager.list_configs(Some(temp_dir.path())).unwrap();
        assert_eq!(configs.len(), 0);
    }

    #[test]
    fn test_list_configs_with_valid_files() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SessionManager::new();

        // Create valid YAML config files
        let yaml1 = r#"
name: project1
root: ~/project1
windows:
  - editor: vim
"#;
        std::fs::write(temp_dir.path().join("project1.yml"), yaml1).unwrap();

        let yaml2 = r#"
name: project2
root: ~/project2
windows:
  - server: npm start
"#;
        std::fs::write(temp_dir.path().join("project2.yaml"), yaml2).unwrap();

        // Create a non-YAML file that should be ignored
        std::fs::write(temp_dir.path().join("readme.txt"), "Not a config").unwrap();

        let configs = manager.list_configs(Some(temp_dir.path())).unwrap();
        assert_eq!(configs.len(), 2);

        let names: Vec<String> = configs.iter().map(|c| c.name.clone()).collect();
        assert!(names.contains(&"project1".to_string()));
        assert!(names.contains(&"project2".to_string()));
    }

    #[test]
    fn test_list_configs_skips_invalid_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SessionManager::new();

        // Create valid YAML
        let valid = r#"
name: valid
root: ~/valid
windows:
  - editor: vim
"#;
        std::fs::write(temp_dir.path().join("valid.yml"), valid).unwrap();

        // Create invalid YAML
        std::fs::write(
            temp_dir.path().join("invalid.yml"),
            "invalid yaml content {{{",
        )
        .unwrap();

        let configs = manager.list_configs(Some(temp_dir.path())).unwrap();
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].name, "valid");
    }

    #[test]
    fn test_session_name_validation() {
        // Test various session name patterns
        let temp_dir = TempDir::new().unwrap();

        let test_cases = vec![
            ("my-project", true),
            ("web_app", true),
            ("app123", true),
            ("123app", true),
            ("my.project", true),
        ];

        for (name, should_succeed) in test_cases {
            let dir_path = temp_dir.path().join(name);
            std::fs::create_dir(&dir_path).unwrap();

            let result = Config::detect_session_name(Some(&dir_path));
            if should_succeed {
                assert!(result.is_ok(), "Failed for name: {name}");
                assert_eq!(result.unwrap(), name);
            }
        }
    }
}
