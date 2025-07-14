use std::sync::atomic::{AtomicU32, Ordering};
use tempfile::TempDir;
use tmuxrs::tmux::TmuxCommand;

/// Common test utilities for integration tests
///
/// ## Integration Test Environment
/// Integration tests only run when INTEGRATION_TESTS=1 is set
/// This flag enables integration tests that require tmux to be available
///
/// ## Handling Attach Operations in Tests
/// Tests that involve tmux attach operations require special cleanup to prevent
/// hanging in Docker/CI environments. Use `cleanup_after_attach_test()` after
/// any attach-related operations to ensure clean state between tests.
///
/// ## Automatic Cleanup
/// All `TmuxTestSession` instances automatically clean up via Rust's Drop trait,
/// but attach operations may require additional server-wide cleanup.
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

        // Note: We don't do aggressive tmux kill-server here anymore.
        // Use cleanup_after_attach_test() explicitly for attach operations.
        // This prevents interference between unrelated tests.
    }
}

/// Gentle cleanup of tmux sessions after attach operations
///
/// This function should be called after any test that performs tmux attach operations,
/// regardless of whether the attach succeeds or fails. Attach operations can leave
/// tmux in an inconsistent state, especially in Docker environments where TTY
/// behavior differs from local execution.
///
/// ## When to use:
/// - After calling `TmuxCommand::attach_session()`
/// - After calling `SessionManager::start_session_with_options()` with `attach=true`
/// - After any interactive tmux operations that might hang
/// - When a test involves session attachment logic
///
/// ## What it does:
/// 1. Only performs cleanup if tmux server appears to be in an inconsistent state
/// 2. Detects hanging attach processes or zombie sessions
/// 3. Preserves sessions that are still actively being tested
/// 4. Only kills the entire server as a last resort
///
/// ## Example usage:
/// ```rust
/// let result = TmuxCommand::attach_session(session.name());
/// match result {
///     Ok(_) => {
///         println!("Attach succeeded");
///         cleanup_after_attach_test(); // Always cleanup after attach
///     }
///     Err(e) => {
///         println!("Attach failed: {}", e);
///         cleanup_after_attach_test(); // Cleanup even on failure
///     }
/// }
/// ```
#[allow(dead_code)]
pub fn cleanup_after_attach_test() {
    if std::env::var("INTEGRATION_TESTS").is_ok() {
        eprintln!("🧹 Checking tmux state after attach operation");

        // Only do aggressive cleanup if we detect hanging processes or inconsistent state
        let tmux_processes = std::process::Command::new("pgrep")
            .args(["-f", "tmux"])
            .output();

        match tmux_processes {
            Ok(output) if !output.stdout.is_empty() => {
                let process_list = String::from_utf8_lossy(&output.stdout);
                let process_count = process_list.lines().count();

                // If there are many tmux processes, it might indicate hanging
                if process_count > 3 {
                    eprintln!(
                        "⚠️ Detected {process_count} tmux processes, checking for cleanup need"
                    );

                    // Try to gracefully check tmux server state
                    let list_result = std::process::Command::new("tmux")
                        .args(["list-sessions"])
                        .output();

                    match list_result {
                        Ok(list_output) if !list_output.stdout.is_empty() => {
                            eprintln!("Active sessions:");
                            eprintln!("{}", String::from_utf8_lossy(&list_output.stdout));
                        }
                        Err(_) => {
                            eprintln!("⚠️ Tmux server appears unresponsive, performing cleanup");
                            let _ = std::process::Command::new("tmux")
                                .args(["kill-server"])
                                .output();
                        }
                        _ => {}
                    }
                } else {
                    eprintln!("✅ Tmux state appears normal ({process_count} processes)");
                }
            }
            _ => {
                eprintln!("✅ No tmux processes detected");
            }
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
