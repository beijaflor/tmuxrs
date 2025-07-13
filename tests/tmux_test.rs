use tmuxrs::error::TmuxrsError;
use tmuxrs::tmux::TmuxCommand;

mod common;
use common::{should_run_integration_tests, TmuxTestSession};

#[test]
fn test_tmux_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
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
        Err(e) => panic!("Unexpected error type: {e}"),
    }
}

#[test]
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session_name = "test-nonexistent-session-12345";

    let exists = TmuxCommand::session_exists(session_name).unwrap();
    assert!(!exists, "Session should not exist");
}

#[test]
fn test_create_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("create-session");

    // Create session
    let result = session.create();
    assert!(result.is_ok(), "Failed to create session: {result:?}");

    // Verify session exists
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after creation");

    // No manual cleanup needed - Drop will handle it
}

#[test]
fn test_create_window() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("create-window");

    // Create session and window
    session.create().unwrap();
    let result = session.create_window("test-window");
    assert!(result.is_ok(), "Failed to create window: {result:?}");

    // No manual cleanup needed - Drop will handle it
}

#[test]
fn test_send_keys() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("send-keys");

    // Create session and window
    session.create().unwrap();
    session.create_window("test-window").unwrap();

    // Send keys
    let result = session.send_keys("test-window", "echo hello");
    assert!(result.is_ok(), "Failed to send keys: {result:?}");

    // No manual cleanup needed - Drop will handle it
}
