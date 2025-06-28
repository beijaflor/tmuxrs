use tempfile::TempDir;
use tmuxrs::error::TmuxrsError;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_tmux_command_execution() {
    // Skip tmux tests in CI environment
    if std::env::var("CI").is_ok() {
        return;
    }
    
    // Test basic tmux command execution
    let result = TmuxCommand::new().arg("list-sessions").execute();

    // Should either succeed or fail with a known error
    match result {
        Ok(_) => {
            // tmux is available and working
        }
        Err(TmuxrsError::TmuxError(_)) => {
            // tmux command failed (expected if no sessions exist)
        }
        Err(e) => panic!("Unexpected error type: {}", e),
    }
}

#[test]
fn test_session_exists_check() {
    // Skip tmux tests in CI environment
    if std::env::var("CI").is_ok() {
        return;
    }
    let session_name = "test-nonexistent-session-12345";

    let exists = TmuxCommand::session_exists(session_name).unwrap();
    assert!(!exists, "Session should not exist");
}

#[test]
fn test_create_session() {
    // Skip tmux tests in CI environment
    if std::env::var("CI").is_ok() {
        return;
    }
    let session_name = "test-create-session-12345";
    let temp_dir = TempDir::new().unwrap();

    // Clean up any existing session first
    let _ = TmuxCommand::kill_session(session_name);

    // Create session
    let result = TmuxCommand::new_session(session_name, temp_dir.path());
    assert!(result.is_ok(), "Failed to create session: {:?}", result);

    // Verify session exists
    let exists = TmuxCommand::session_exists(session_name).unwrap();
    assert!(exists, "Session should exist after creation");

    // Clean up
    let _ = TmuxCommand::kill_session(session_name);
}

#[test]
fn test_create_window() {
    // Skip tmux tests in CI environment
    if std::env::var("CI").is_ok() {
        return;
    }
    let session_name = "test-window-session-12345";
    let temp_dir = TempDir::new().unwrap();

    // Clean up and create session
    let _ = TmuxCommand::kill_session(session_name);
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Create window
    let result = TmuxCommand::new_window(session_name, "test-window", None);
    assert!(result.is_ok(), "Failed to create window: {:?}", result);

    // Clean up
    let _ = TmuxCommand::kill_session(session_name);
}

#[test]
fn test_send_keys() {
    // Skip tmux tests in CI environment
    if std::env::var("CI").is_ok() {
        return;
    }
    let session_name = "test-keys-session-12345";
    let temp_dir = TempDir::new().unwrap();

    // Clean up and create session
    let _ = TmuxCommand::kill_session(session_name);
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Create window first
    TmuxCommand::new_window(session_name, "test-window", None).unwrap();

    // Send keys
    let result = TmuxCommand::send_keys(session_name, "test-window", "echo hello");
    assert!(result.is_ok(), "Failed to send keys: {:?}", result);

    // Clean up
    let _ = TmuxCommand::kill_session(session_name);
}
