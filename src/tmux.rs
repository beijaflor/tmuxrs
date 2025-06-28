use crate::error::{Result, TmuxrsError};
use std::path::Path;
use std::process::Command;

/// Wrapper for tmux command execution
#[derive(Default)]
#[allow(dead_code)]
pub struct TmuxCommand {
    args: Vec<String>,
}

impl TmuxCommand {
    /// Create a new tmux command
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an argument to the command
    #[allow(dead_code)]
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Execute the tmux command
    #[allow(dead_code)]
    pub fn execute(self) -> Result<String> {
        let output = Command::new("tmux")
            .args(&self.args)
            .output()
            .map_err(|e| TmuxrsError::TmuxError(format!("Failed to execute tmux: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TmuxrsError::TmuxError(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Check if a session exists
    #[allow(dead_code)]
    pub fn session_exists(session_name: &str) -> Result<bool> {
        let result = Self::new()
            .arg("has-session")
            .arg("-t")
            .arg(session_name)
            .execute();

        match result {
            Ok(_) => Ok(true),
            Err(TmuxrsError::TmuxError(_)) => Ok(false), // Session doesn't exist
            Err(e) => Err(e),                            // Other error
        }
    }

    /// Create a new tmux session
    #[allow(dead_code)]
    pub fn new_session(session_name: &str, working_dir: &Path) -> Result<String> {
        Self::new()
            .arg("new-session")
            .arg("-d") // Detached
            .arg("-s")
            .arg(session_name)
            .arg("-c")
            .arg(working_dir.to_string_lossy().as_ref())
            .execute()
    }

    /// Create a new window in a session
    #[allow(dead_code)]
    pub fn new_window(
        session_name: &str,
        window_name: &str,
        command: Option<&str>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("new-window")
            .arg("-t")
            .arg(session_name)
            .arg("-n")
            .arg(window_name);

        if let Some(cmd_str) = command {
            cmd = cmd.arg(cmd_str);
        }

        cmd.execute()
    }

    /// Send keys to a window
    #[allow(dead_code)]
    pub fn send_keys(session_name: &str, window_name: &str, keys: &str) -> Result<String> {
        Self::new()
            .arg("send-keys")
            .arg("-t")
            .arg(format!("{}:{}", session_name, window_name))
            .arg(keys)
            .arg("Enter")
            .execute()
    }

    /// Kill a session
    #[allow(dead_code)]
    pub fn kill_session(session_name: &str) -> Result<String> {
        Self::new()
            .arg("kill-session")
            .arg("-t")
            .arg(session_name)
            .execute()
    }
}
