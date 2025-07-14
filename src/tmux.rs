use crate::error::{Result, TmuxrsError};
use std::path::Path;
use std::process::{Command, Stdio};

/// Wrapper for tmux command execution
#[derive(Default)]
#[allow(dead_code)]
pub struct TmuxCommand {
    args: Vec<String>,
    socket_path: Option<String>,
}

impl TmuxCommand {
    /// Create a new tmux command
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new tmux command with a custom socket path
    #[allow(dead_code)]
    pub fn with_socket<P: AsRef<Path>>(socket_path: P) -> Self {
        Self {
            args: Vec::new(),
            socket_path: Some(socket_path.as_ref().to_string_lossy().to_string()),
        }
    }

    /// Set the socket path for this command
    #[allow(dead_code)]
    pub fn socket<P: AsRef<Path>>(mut self, socket_path: P) -> Self {
        self.socket_path = Some(socket_path.as_ref().to_string_lossy().to_string());
        self
    }

    /// Add an argument to the command
    #[allow(dead_code)]
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Execute the tmux command (non-interactive)
    #[allow(dead_code)]
    pub fn execute(self) -> Result<String> {
        let mut cmd = Command::new("tmux");

        // Add socket path if specified
        if let Some(socket) = &self.socket_path {
            cmd.args(["-S", socket]);
        }

        let output = cmd
            .args(&self.args)
            .output()
            .map_err(|e| TmuxrsError::TmuxError(format!("Failed to execute tmux: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TmuxrsError::TmuxError(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute tmux command interactively (inherits TTY for attach-session)
    #[allow(dead_code)]
    pub fn execute_interactive(self) -> Result<()> {
        // Check if we're in a TTY environment - if not, return an error instead of hanging
        if !Self::is_tty_available() {
            return Err(TmuxrsError::TmuxError(
                "Failed to attach: No TTY available (running in non-interactive environment like Docker)".to_string()
            ));
        }

        let mut cmd = Command::new("tmux");

        // Add socket path if specified
        if let Some(socket) = &self.socket_path {
            cmd.args(["-S", socket]);
        }

        let mut child = cmd
            .args(&self.args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| TmuxrsError::TmuxError(format!("Failed to execute tmux: {e}")))?;

        let status = child
            .wait()
            .map_err(|e| TmuxrsError::TmuxError(format!("Failed to wait for tmux: {e}")))?;

        if !status.success() {
            return Err(TmuxrsError::TmuxError(format!(
                "tmux command failed with exit code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    /// Check if TTY is available for interactive operations
    #[allow(dead_code)]
    fn is_tty_available() -> bool {
        use std::io::IsTerminal;
        std::io::stdin().is_terminal()
    }

    /// Check if a session exists
    #[allow(dead_code)]
    pub fn session_exists(session_name: &str) -> Result<bool> {
        Self::session_exists_with_socket(session_name, None::<&Path>)
    }

    /// Check if a session exists using a specific socket
    #[allow(dead_code)]
    pub fn session_exists_with_socket<P: AsRef<Path>>(
        session_name: &str,
        socket_path: Option<P>,
    ) -> Result<bool> {
        let mut cmd = Self::new().arg("has-session").arg("-t").arg(session_name);

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        let result = cmd.execute();

        match result {
            Ok(_) => Ok(true),
            Err(TmuxrsError::TmuxError(_)) => Ok(false), // Session doesn't exist
            Err(e) => Err(e),                            // Other error
        }
    }

    /// Create a new tmux session
    #[allow(dead_code)]
    pub fn new_session(session_name: &str, working_dir: &Path) -> Result<String> {
        Self::new_session_with_socket(session_name, working_dir, None::<&Path>)
    }

    /// Create a new tmux session using a specific socket
    #[allow(dead_code)]
    pub fn new_session_with_socket<P: AsRef<Path>>(
        session_name: &str,
        working_dir: &Path,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("new-session")
            .arg("-d") // Detached
            .arg("-s")
            .arg(session_name)
            .arg("-c")
            .arg(working_dir.to_string_lossy().as_ref());

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Create a new window in a session
    #[allow(dead_code)]
    pub fn new_window(
        session_name: &str,
        window_name: &str,
        command: Option<&str>,
        working_dir: Option<&Path>,
    ) -> Result<String> {
        Self::new_window_with_socket(
            session_name,
            window_name,
            command,
            working_dir,
            None::<&Path>,
        )
    }

    /// Create a new window in a session using a specific socket
    #[allow(dead_code)]
    pub fn new_window_with_socket<P: AsRef<Path>>(
        session_name: &str,
        window_name: &str,
        command: Option<&str>,
        working_dir: Option<&Path>,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("new-window")
            .arg("-t")
            .arg(session_name)
            .arg("-n")
            .arg(window_name);

        // Add working directory if provided
        if let Some(dir) = working_dir {
            cmd = cmd.arg("-c").arg(dir.to_string_lossy().as_ref());
        }

        if let Some(cmd_str) = command {
            cmd = cmd.arg(cmd_str);
        }

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Send keys to a window
    #[allow(dead_code)]
    pub fn send_keys(session_name: &str, window_name: &str, keys: &str) -> Result<String> {
        Self::send_keys_with_socket(session_name, window_name, keys, None::<&Path>)
    }

    /// Send keys to a window using a specific socket
    #[allow(dead_code)]
    pub fn send_keys_with_socket<P: AsRef<Path>>(
        session_name: &str,
        window_name: &str,
        keys: &str,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("send-keys")
            .arg("-t")
            .arg(format!("{session_name}:{window_name}"))
            .arg(keys)
            .arg("Enter");

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Send keys to a specific pane
    #[allow(dead_code)]
    pub fn send_keys_to_pane(
        session_name: &str,
        window_name: &str,
        pane_index: usize,
        keys: &str,
    ) -> Result<String> {
        Self::new()
            .arg("send-keys")
            .arg("-t")
            .arg(format!("{session_name}:{window_name}.{pane_index}"))
            .arg(keys)
            .arg("Enter")
            .execute()
    }

    /// Kill a session
    #[allow(dead_code)]
    pub fn kill_session(session_name: &str) -> Result<String> {
        Self::kill_session_with_socket(session_name, None::<&Path>)
    }

    /// Kill a session using a specific socket
    #[allow(dead_code)]
    pub fn kill_session_with_socket<P: AsRef<Path>>(
        session_name: &str,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new().arg("kill-session").arg("-t").arg(session_name);

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Split window horizontally (side by side)
    #[allow(dead_code)]
    pub fn split_window_horizontal(
        session_name: &str,
        window_name: &str,
        command: &str,
        working_dir: Option<&Path>,
    ) -> Result<String> {
        Self::split_window_horizontal_with_socket(
            session_name,
            window_name,
            command,
            working_dir,
            None::<&Path>,
        )
    }

    /// Split window horizontally using a specific socket
    #[allow(dead_code)]
    pub fn split_window_horizontal_with_socket<P: AsRef<Path>>(
        session_name: &str,
        window_name: &str,
        command: &str,
        working_dir: Option<&Path>,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("split-window")
            .arg("-h") // horizontal split (side by side)
            .arg("-t")
            .arg(if window_name.is_empty() {
                session_name.to_string()
            } else {
                format!("{session_name}:{window_name}")
            });

        // Add working directory if provided
        if let Some(dir) = working_dir {
            cmd = cmd.arg("-c").arg(dir.to_string_lossy().as_ref());
        }

        // Only add command if not empty - this allows shell to initialize properly
        if !command.trim().is_empty() {
            cmd = cmd.arg(command);
        }

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Split window vertically (above/below)
    #[allow(dead_code)]
    pub fn split_window_vertical(
        session_name: &str,
        window_name: &str,
        command: &str,
        working_dir: Option<&Path>,
    ) -> Result<String> {
        Self::split_window_vertical_with_socket(
            session_name,
            window_name,
            command,
            working_dir,
            None::<&Path>,
        )
    }

    /// Split window vertically using a specific socket
    #[allow(dead_code)]
    pub fn split_window_vertical_with_socket<P: AsRef<Path>>(
        session_name: &str,
        window_name: &str,
        command: &str,
        working_dir: Option<&Path>,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("split-window")
            .arg("-v") // vertical split (above/below)
            .arg("-t")
            .arg(if window_name.is_empty() {
                session_name.to_string()
            } else {
                format!("{session_name}:{window_name}")
            });

        // Add working directory if provided
        if let Some(dir) = working_dir {
            cmd = cmd.arg("-c").arg(dir.to_string_lossy().as_ref());
        }

        // Only add command if not empty - this allows shell to initialize properly
        if !command.trim().is_empty() {
            cmd = cmd.arg(command);
        }

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Select layout for a window
    #[allow(dead_code)]
    pub fn select_layout(session_name: &str, window_name: &str, layout: &str) -> Result<String> {
        Self::select_layout_with_socket(session_name, window_name, layout, None::<&Path>)
    }

    /// Select layout for a window using a specific socket
    #[allow(dead_code)]
    pub fn select_layout_with_socket<P: AsRef<Path>>(
        session_name: &str,
        window_name: &str,
        layout: &str,
        socket_path: Option<P>,
    ) -> Result<String> {
        let mut cmd = Self::new()
            .arg("select-layout")
            .arg("-t")
            .arg(if window_name.is_empty() {
                session_name.to_string()
            } else {
                format!("{session_name}:{window_name}")
            })
            .arg(layout);

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }

    /// Attach to a session (interactive)
    #[allow(dead_code)]
    pub fn attach_session(session_name: &str) -> Result<()> {
        Self::attach_session_with_socket(session_name, None::<&Path>)
    }

    /// Attach to a session using a specific socket (interactive)
    #[allow(dead_code)]
    pub fn attach_session_with_socket<P: AsRef<Path>>(
        session_name: &str,
        socket_path: Option<P>,
    ) -> Result<()> {
        let mut cmd = Self::new()
            .arg("attach-session")
            .arg("-t")
            .arg(session_name);

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute_interactive()
    }

    /// Kill the tmux server
    #[allow(dead_code)]
    pub fn kill_server() -> Result<String> {
        Self::kill_server_with_socket(None::<&Path>)
    }

    /// Kill the tmux server using a specific socket
    #[allow(dead_code)]
    pub fn kill_server_with_socket<P: AsRef<Path>>(socket_path: Option<P>) -> Result<String> {
        let mut cmd = Self::new().arg("kill-server");

        if let Some(socket) = socket_path {
            cmd = cmd.socket(socket);
        }

        cmd.execute()
    }
}
