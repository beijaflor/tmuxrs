use std::sync::atomic::{AtomicU32, Ordering};
use tempfile::TempDir;
use tmuxrs::tmux::TmuxCommand;

/// Common test utilities for integration tests
///
/// Check if integration tests should run
/// Integration tests only run when INTEGRATION_TESTS=1 is set
/// This flag enables integration tests that require tmux to be available
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
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("tmuxrs-test-{test_name}-{timestamp}-{counter}")
}

/// Automatic cleanup for tmux test sessions
///
/// This struct ensures that tmux sessions created during tests are always
/// cleaned up, even if the test panics or fails.
#[allow(dead_code)]
pub struct TmuxTestSession {
    session_name: String,
    temp_dir: Option<TempDir>,
}

#[allow(dead_code)]
impl TmuxTestSession {
    /// Create a new test session with automatic cleanup
    pub fn new(test_name: &str) -> Self {
        let session_name = generate_test_session_name(test_name);

        // Clean up any existing session with this name (shouldn't happen, but just in case)
        let _ = TmuxCommand::kill_session(&session_name);

        Self {
            session_name,
            temp_dir: None,
        }
    }

    /// Create a new test session with a temporary directory
    pub fn with_temp_dir(test_name: &str) -> Self {
        let mut session = Self::new(test_name);
        session.temp_dir = Some(TempDir::new().expect("Failed to create temp directory"));
        session
    }

    /// Get the session name
    pub fn name(&self) -> &str {
        &self.session_name
    }

    /// Get the temporary directory if one was created
    pub fn temp_dir(&self) -> Option<&std::path::Path> {
        self.temp_dir.as_ref().map(|d| d.path())
    }

    /// Create the tmux session
    pub fn create(&self) -> Result<String, tmuxrs::error::TmuxrsError> {
        let working_dir = self.temp_dir().unwrap_or_else(|| std::path::Path::new("."));
        TmuxCommand::new_session(&self.session_name, working_dir)
    }

    /// Check if the session exists
    pub fn exists(&self) -> Result<bool, tmuxrs::error::TmuxrsError> {
        TmuxCommand::session_exists(&self.session_name)
    }

    /// Create a new window in the session
    pub fn create_window(&self, window_name: &str) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::new_window(&self.session_name, window_name, None, None)
    }

    /// Send keys to a window
    pub fn send_keys(
        &self,
        window_name: &str,
        keys: &str,
    ) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::send_keys(&self.session_name, window_name, keys)
    }

    /// Manually kill the session (usually not needed due to Drop)
    pub fn kill(&self) -> Result<String, tmuxrs::error::TmuxrsError> {
        TmuxCommand::kill_session(&self.session_name)
    }
}

impl Drop for TmuxTestSession {
    fn drop(&mut self) {
        // Always attempt cleanup, but don't panic on errors
        if let Err(e) = TmuxCommand::kill_session(&self.session_name) {
            eprintln!(
                "Warning: Failed to cleanup test session '{}': {}",
                self.session_name, e
            );
        }
    }
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
