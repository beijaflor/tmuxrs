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

    // Test basic tmux command execution with list-sessions
    let result = TmuxCommand::new().arg("list-sessions").execute();

    // Both success and "no sessions" error are valid outcomes
    match result {
        Ok(output) => {
            println!("✓ Successfully executed tmux command: {output}");
            assert!(
                output.contains("tmux") || output.is_empty(),
                "Output should be valid tmux response"
            );
        }
        Err(TmuxrsError::TmuxError(msg)) => {
            // This is expected when no sessions exist
            println!("✓ Tmux command failed as expected (no sessions): {msg}");
            assert!(
                msg.contains("no server running") || msg.contains("failed to connect"),
                "Error should indicate no tmux server: {msg}"
            );
        }
        Err(other) => {
            panic!("Unexpected error type: {other:?}");
        }
    }

    println!("✓ Tmux command execution test passed");
}

/// Tests for session existence checking
#[test]
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    // Test checking for a session that definitely doesn't exist
    let session_name = "test-nonexistent-session-12345";
    let result = TmuxCommand::session_exists(session_name);

    assert!(
        result.is_ok(),
        "Session existence check should not fail: {result:?}"
    );

    let exists = result.unwrap();
    assert!(!exists, "Non-existent session should return false");

    println!("✓ Session existence check test passed");
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

    println!("✓ Session creation test passed");
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

    println!("✓ Window creation test passed");
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

    println!("✓ Send keys test passed");
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

    println!("✓ Isolated server operations test passed");
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
            // Unexpected success - server might have had existing sessions
            println!("⚠ Unexpected success - isolated server had existing sessions");
        }
        Err(TmuxrsError::TmuxError(msg)) => {
            // Expected error for empty tmux server
            println!("✓ Got expected TmuxError for empty server: {msg}");
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
            // Some other error occurred
            println!("⚠ Got unexpected error type: {other:?}");
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

    println!("✓ Command building and error handling test passed");
    // Automatic cleanup via Drop trait
}
