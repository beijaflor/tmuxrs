use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use tempfile::TempDir;
use tmuxrs::tmux::TmuxCommand;

/// Common test utilities for integration tests with isolated tmux servers
///
/// ## Integration Test Environment
/// Integration tests only run when INTEGRATION_TESTS=1 is set
/// This flag enables integration tests that require tmux to be available
///
/// ## Test Isolation
/// Each `TmuxTestSession` creates its own isolated tmux server with a unique socket path.
/// This eliminates test interference and the need for complex cleanup logic.
/// Tests can run in parallel without affecting each other.
///
/// ## Automatic Cleanup
/// All `TmuxTestSession` instances automatically clean up their isolated tmux server
/// via Rust's Drop trait. No manual cleanup is required.
pub fn should_run_integration_tests() -> bool {
    std::env::var("INTEGRATION_TESTS").is_ok()
}

/// Atomic counter for generating unique session names
#[allow(dead_code)]
static SESSION_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Generate a unique session name for testing
#[allow(dead_code)]
pub fn generate_test_session_name(test_name: &str) -> String {
    let counter = SESSION_COUNTER.fetch_add(1, Ordering::SeqCst);
    // Use process ID and counter for shorter, still unique names
    let pid = std::process::id();
    format!("tmx-{test_name}-{pid}-{counter}")
}

/// Automatic cleanup for tmux test sessions with isolated tmux servers
///
/// This struct ensures that tmux sessions created during tests are always
/// cleaned up, even if the test panics or fails. Each test session gets its
/// own isolated tmux server with a unique socket path, preventing interference
/// between tests.
#[allow(dead_code)]
pub struct TmuxTestSession {
    session_name: String,
    temp_dir: Option<TempDir>,
    socket_path: PathBuf,
    socket_dir: TempDir,
}

#[allow(dead_code)]
impl TmuxTestSession {
    /// Create a new test session with automatic cleanup and isolated tmux server
    pub fn new(test_name: &str) -> Self {
        let session_name = generate_test_session_name(test_name);

        // Create a unique socket directory for this test
        let socket_dir = TempDir::new().expect("Failed to create socket directory");
        // Use a very short socket name to avoid path length limits
        let socket_path = socket_dir
            .path()
            .join(format!("s{}", SESSION_COUNTER.load(Ordering::SeqCst)));

        Self {
            session_name,
            temp_dir: None,
            socket_path,
            socket_dir,
        }
    }

    /// Create a new test session with a temporary directory and isolated tmux server
    pub fn with_temp_dir(test_name: &str) -> Self {
        let mut session = Self::new(test_name);
        session.temp_dir = Some(TempDir::new().expect("Failed to create temp directory"));
        session
    }

    /// Get the session name
    pub fn name(&self) -> &str {
        &self.session_name
    }

    /// Get the socket path for this isolated tmux server
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Get the temporary directory if one was created
    pub fn temp_dir(&self) -> Option<&std::path::Path> {
        self.temp_dir.as_ref().map(|d| d.path())
    }

    /// Create the tmux session using isolated server
    pub fn create(&self) -> Result<String, tmuxrs::error::TmuxrsError> {
        let working_dir = self.temp_dir().unwrap_or_else(|| std::path::Path::new("."));
        TmuxCommand::new_session_with_socket(
            &self.session_name,
            working_dir,
            Some(&self.socket_path),
        )
    }

    /// Check if the session exists on the isolated server
    pub fn exists(&self) -> Result<bool, tmuxrs::error::TmuxrsError> {
        TmuxCommand::session_exists_with_socket(&self.session_name, Some(&self.socket_path))
    }

    /// Create a new window in the session on the isolated server
    pub fn create_window(&self, window_name: &str) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::new_window_with_socket(
            &self.session_name,
            window_name,
            None,
            None,
            Some(&self.socket_path),
        )
    }

    /// Send keys to a window on the isolated server
    pub fn send_keys(
        &self,
        window_name: &str,
        keys: &str,
    ) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::send_keys_with_socket(
            &self.session_name,
            window_name,
            keys,
            Some(&self.socket_path),
        )
    }

    /// Manually kill the session on the isolated server (usually not needed due to Drop)
    pub fn kill(&self) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::kill_session_with_socket(&self.session_name, Some(&self.socket_path))
    }

    /// Split window horizontally on the isolated server
    pub fn split_window_horizontal(
        &self,
        window_name: &str,
        command: &str,
    ) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::split_window_horizontal_with_socket(
            &self.session_name,
            window_name,
            command,
            None,
            Some(&self.socket_path),
        )
    }

    /// Split window vertically on the isolated server
    pub fn split_window_vertical(
        &self,
        window_name: &str,
        command: &str,
    ) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::split_window_vertical_with_socket(
            &self.session_name,
            window_name,
            command,
            None,
            Some(&self.socket_path),
        )
    }

    /// Select layout for a window on the isolated server
    pub fn select_layout(
        &self,
        window_name: &str,
        layout: &str,
    ) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::select_layout_with_socket(
            &self.session_name,
            window_name,
            layout,
            Some(&self.socket_path),
        )
    }
}

impl Drop for TmuxTestSession {
    fn drop(&mut self) {
        // Kill the entire isolated tmux server - this is safe since each test
        // has its own server and won't affect other tests
        if let Err(e) = TmuxCommand::kill_server_with_socket(Some(&self.socket_path)) {
            eprintln!(
                "Warning: Failed to cleanup isolated tmux server for '{}': {}",
                self.session_name, e
            );
        }

        // The socket_dir TempDir will be automatically cleaned up when dropped
    }
}

/// Legacy cleanup function - no longer needed with isolated tmux servers
///
/// With the new isolated tmux server approach, each test has its own tmux server
/// with a unique socket path. This eliminates the need for complex cleanup logic
/// since tests cannot interfere with each other.
///
/// This function is kept for backward compatibility but is now a no-op.
/// Test isolation is automatically handled by `TmuxTestSession::Drop`.
#[allow(dead_code)]
pub fn cleanup_after_attach_test() {
    // No-op: Test isolation eliminates the need for manual cleanup
}

/// Skip test if not in proper environment
#[macro_export]
macro_rules! skip_if_not_integration_env {
    () => {
        if !$crate::common::should_run_integration_tests() {
            eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
            return;
        }
    };
}
