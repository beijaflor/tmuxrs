use crate::common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::error::TmuxrsError;
use tmuxrs::tmux::TmuxCommand;

/// Tests for basic tmux command execution
#[test]
fn test_tmux_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    // Create an isolated test environment
    let session = TmuxTestSession::new("command-execution");

    // Test basic tmux command execution with list-sessions on isolated server
    let result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-sessions")
        .execute();

    // Both success and "no sessions" error are valid outcomes
    match result {
        Ok(_output) => {
            // Accept any output as valid since list-sessions will show existing sessions
            // or be empty if none exist. The command succeeded, that's what matters.
        }
        Err(TmuxrsError::TmuxError(msg)) => {
            // This is expected when no sessions exist on the isolated server
            assert!(
                msg.contains("no server running")
                    || msg.contains("failed to connect")
                    || msg.contains("no sessions")
                    || msg.contains("error connecting to")
                    || msg.contains("No such file or directory"),
                "Error should indicate no tmux server or sessions: {msg}"
            );
        }
        Err(other) => {
            panic!("Unexpected error type: {other:?}");
        }
    }
}

/// Tests for session existence checking
#[test]
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    // Create an isolated test environment
    let session = TmuxTestSession::new("nonexistent-check");

    // Test checking for a session that definitely doesn't exist
    let session_name = "test-nonexistent-session-12345";
    let result = TmuxCommand::session_exists_with_socket(session_name, Some(session.socket_path()));

    assert!(
        result.is_ok(),
        "Session existence check should not fail: {result:?}"
    );

    let exists = result.unwrap();
    assert!(!exists, "Non-existent session should return false");
}

/// Tests for tmux session creation using isolated servers
#[test]
fn test_create_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("tmux-create-session");

    // Create session using isolated tmux server
    let create_result = session.create();
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Verify session exists on the isolated server
    let exists_result = session.exists();
    assert!(
        exists_result.is_ok(),
        "Failed to check session existence: {exists_result:?}"
    );

    let exists = exists_result.unwrap();
    assert!(exists, "Session should exist after creation");

    // Automatic cleanup via Drop trait
}

/// Tests for window creation within existing sessions
#[test]
fn test_create_window() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("tmux-create-window");

    // Create session first
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    // Create window in the session
    let window_result = session.create_window("test-window");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Verify window was created by checking if session still exists
    // (more detailed window verification would require additional tmux commands)
    let exists = session.exists().unwrap();
    assert!(exists, "Session should still exist after window creation");

    // Automatic cleanup via Drop trait
}

/// Tests for sending key commands to tmux windows
#[test]
fn test_send_keys() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("tmux-send-keys");

    // Create session and window
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    let window_result = session.create_window("test-window");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Send keys to the window
    let keys_result = session.send_keys("test-window", "echo hello");
    assert!(keys_result.is_ok(), "Failed to send keys: {keys_result:?}");

    // Verify the session and window still exist after sending keys
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after sending keys");

    // Automatic cleanup via Drop trait
}

/// Tests for advanced tmux operations with isolated servers
#[test]
fn test_isolated_server_operations() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("isolated-operations");

    // Create session
    let create_result = session.create();
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Test multiple windows
    let window1_result = session.create_window("window1");
    assert!(
        window1_result.is_ok(),
        "Failed to create window1: {window1_result:?}"
    );

    let window2_result = session.create_window("window2");
    assert!(
        window2_result.is_ok(),
        "Failed to create window2: {window2_result:?}"
    );

    // Test sending keys to different windows
    let keys1_result = session.send_keys("window1", "echo 'window 1'");
    assert!(
        keys1_result.is_ok(),
        "Failed to send keys to window1: {keys1_result:?}"
    );

    let keys2_result = session.send_keys("window2", "echo 'window 2'");
    assert!(
        keys2_result.is_ok(),
        "Failed to send keys to window2: {keys2_result:?}"
    );

    // Test window splitting
    let split_h_result = session.split_window_horizontal("window1", "echo 'split horizontal'");
    assert!(
        split_h_result.is_ok(),
        "Failed to split horizontally: {split_h_result:?}"
    );

    let split_v_result = session.split_window_vertical("window2", "echo 'split vertical'");
    assert!(
        split_v_result.is_ok(),
        "Failed to split vertically: {split_v_result:?}"
    );

    // Test layout selection
    let layout_result = session.select_layout("window1", "main-vertical");
    assert!(
        layout_result.is_ok(),
        "Failed to select layout: {layout_result:?}"
    );

    // Verify session still exists after all operations
    let final_exists = session.exists().unwrap();
    assert!(final_exists, "Session should exist after all operations");

    // Automatic cleanup via Drop trait will clean up the entire isolated server
}

/// Tests for tmux command building and error handling
#[test]
fn test_command_building_and_error_handling() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    // Test command building with socket path
    let session = TmuxTestSession::new("command-building");

    // Test that commands can be built with socket paths
    let cmd = TmuxCommand::with_socket(session.socket_path()).arg("list-sessions");

    // Execute the command (should fail since no sessions exist in new server)
    let result = cmd.execute();

    // Should get a TmuxError since the isolated server has no sessions
    match result {
        Ok(_) => {
            panic!("Unexpected success - isolated server should not have existing sessions");
        }
        Err(TmuxrsError::TmuxError(msg)) => {
            // Expected error for empty tmux server
            assert!(
                msg.contains("no server running")
                    || msg.contains("failed to connect")
                    || msg.contains("can't find")
                    || msg.contains("error connecting to")
                    || msg.contains("No such file or directory"),
                "Error should indicate no sessions/server: {msg}"
            );
        }
        Err(other) => {
            panic!("Got unexpected error type: {other:?}");
        }
    }

    // Now create a session and test successful command execution
    let create_result = session.create();
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Test list-sessions on server with sessions
    let list_cmd = TmuxCommand::with_socket(session.socket_path()).arg("list-sessions");

    let list_result = list_cmd.execute();
    assert!(
        list_result.is_ok(),
        "List sessions should succeed with existing session: {list_result:?}"
    );

    let output = list_result.unwrap();
    assert!(
        output.contains(session.name()),
        "Output should contain session name: {output}"
    );

    // Automatic cleanup via Drop trait
}
