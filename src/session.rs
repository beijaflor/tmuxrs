use crate::config::Config;
use crate::error::{Result, TmuxrsError};
use crate::tmux::TmuxCommand;
use std::path::Path;

/// Session manager for tmuxrs
#[derive(Default)]
pub struct SessionManager;

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self
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
        let root_path = Path::new(root_dir);
        TmuxCommand::new_session(&session_name, root_path)?;

        // Create windows
        for (index, window_config) in config.windows.iter().enumerate() {
            match window_config {
                crate::config::WindowConfig::Simple(command) => {
                    let window_name = format!("window-{}", index + 1);
                    TmuxCommand::new_window(
                        &session_name,
                        &window_name,
                        Some(command),
                        Some(root_path),
                    )?;
                }
                crate::config::WindowConfig::Complex { window } => {
                    for (window_name, command) in window {
                        TmuxCommand::new_window(
                            &session_name,
                            window_name,
                            Some(command),
                            Some(root_path),
                        )?;
                    }
                }
                crate::config::WindowConfig::WithLayout { window } => {
                    for (window_name, layout_config) in window {
                        // Create the window with the first pane
                        let first_pane = layout_config.panes.first().ok_or_else(|| {
                            TmuxrsError::TmuxError(
                                "Window layout must have at least one pane".to_string(),
                            )
                        })?;
                        TmuxCommand::new_window(
                            &session_name,
                            window_name,
                            Some(first_pane),
                            Some(root_path),
                        )?;

                        // Add additional panes by splitting
                        for pane_command in layout_config.panes.iter().skip(1) {
                            TmuxCommand::split_window_horizontal(
                                &session_name,
                                window_name,
                                pane_command,
                                Some(root_path),
                            )?;
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
