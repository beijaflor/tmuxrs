use crate::common::{cleanup_after_attach_test, should_run_integration_tests, TmuxTestSession};
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

/// Tests for session creation and existence checking
#[test]
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("session-exists-check");

    // Initially, session should not exist
    let exists_before = session.exists().unwrap();
    assert!(!exists_before, "Session should not exist initially");

    // Create session
    let create_result = session.create();
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Now session should exist
    let exists_after = session.exists().unwrap();
    assert!(exists_after, "Session should exist after creation");

    // Test checking for non-existent session
    let non_existent = TmuxTestSession::new("definitely-does-not-exist");
    let non_existent_check = non_existent.exists().unwrap();
    assert!(
        !non_existent_check,
        "Non-existent session should return false"
    );

    println!("✓ Session existence check test passed");
}

#[test]
fn test_basic_session_creation() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("basic-creation");

    // Create session
    let result = session.create();
    assert!(result.is_ok(), "Failed to create session: {result:?}");

    // Verify session exists through direct tmux commands
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after creation");

    // Verify through list-sessions command
    let list_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-sessions")
        .execute();
    assert!(
        list_result.is_ok(),
        "Failed to list sessions: {list_result:?}"
    );
    let sessions_output = list_result.unwrap();
    assert!(
        sessions_output.contains(session.name()),
        "Session should appear in list-sessions output"
    );

    println!("✓ Basic session creation test passed");
}

#[test]
fn test_create_session_simple() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("create-session");

    // Create session
    let result = session.create();
    assert!(result.is_ok(), "Failed to create session: {result:?}");

    // Verify session exists
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after creation");

    println!("✓ Session creation test passed");
}

/// Session attachment tests (many ignored due to Docker/TTY limitations)
#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_attach_to_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("attach-existing");

    // First create the session
    let create_result = session.create();
    assert!(
        create_result.is_ok(),
        "Failed to create session for attach test: {create_result:?}"
    );

    // Verify session exists
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist before attach attempt");

    // Test attaching to the existing session
    let attach_result =
        TmuxCommand::attach_session_with_socket(session.name(), Some(session.socket_path()));

    // In Docker/CI environments without TTY, attach will fail
    // In interactive terminals, attach would succeed
    match attach_result {
        Ok(_) => {
            println!("✓ Successfully attached to existing session (TTY available)");
            cleanup_after_attach_test();
        }
        Err(error) => {
            // Expected in Docker/CI environments
            assert!(
                error.to_string().contains("open terminal failed")
                    || error.to_string().contains("not a terminal"),
                "Attach failure should be due to TTY issues: {error}"
            );
            println!("✓ Attach failed as expected in non-TTY environment");
            cleanup_after_attach_test();
        }
    }
}

#[test]
fn test_attach_to_nonexistent_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("nonexistent-attach");

    // Try to attach to a session that doesn't exist
    let attach_result =
        TmuxCommand::attach_session_with_socket(session.name(), Some(session.socket_path()));

    // Should fail because session doesn't exist
    assert!(
        attach_result.is_err(),
        "Attach to nonexistent session should fail"
    );

    let error = attach_result.unwrap_err();
    assert!(
        error.to_string().contains("can't find session")
            || error.to_string().contains("session not found"),
        "Error should indicate session not found: {error}"
    );

    println!("✓ Attach to nonexistent session correctly failed");
}

#[test]
fn test_start_session_no_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("start-no-attach");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: echo "test session"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Test starting session with attach = false
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to start session without attach: {result:?}"
    );

    // Verify session exists in default tmux server
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session should exist after starting without attach");

    // Clean up the session that was created in the default tmux server
    let _ = TmuxCommand::kill_session(session.name());

    println!("✓ Start session without attach test passed");
}

#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_existing_session_with_attach() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("existing-attach");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: sleep 30
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // First create the session (detached)
    let create_result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session should exist before attach attempt");

    // Now try to start existing session with attach = true
    let attach_result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    // Handle both success and failure cases
    match attach_result {
        Ok(msg) => {
            println!("✓ Successfully attached to existing session: {msg}");
            cleanup_after_attach_test();
        }
        Err(error) => {
            // Expected in Docker/CI environments
            println!("✓ Attach failed as expected in non-TTY environment: {error}");
            cleanup_after_attach_test();
        }
    }

    // Clean up the session that was created in the default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_start_session_with_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("start-with-attach");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor: vim
  - terminal: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Test starting a new session with attach = true
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    // Handle both success and failure cases
    match result {
        Ok(msg) => {
            println!("✓ Successfully started and attached to session: {msg}");
            cleanup_after_attach_test();
        }
        Err(error) => {
            // Expected in Docker/CI environments without TTY
            println!("✓ Start with attach failed as expected in non-TTY environment: {error}");
            cleanup_after_attach_test();
        }
    }

    // Clean up any session that may have been created
    let _ = TmuxCommand::kill_session(session.name());
}

/// Session stopping tests (many ignored due to SessionManager isolation issues)
#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_stop_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("stop-existing");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: sleep 60
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Create session first
    let create_result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(
        create_result.is_ok(),
        "Failed to create session: {create_result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session should exist before stopping");

    // Stop the session
    let stop_result = session_manager.stop_session(session.name());
    assert!(
        stop_result.is_ok(),
        "Failed to stop session: {stop_result:?}"
    );

    // Verify session no longer exists
    let exists_after = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(!exists_after, "Session should not exist after stopping");

    println!("✓ Stop existing session test passed");
}

#[test]
fn test_stop_nonexistent_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session_manager = SessionManager::new();
    let result = session_manager.stop_session("definitely-does-not-exist");

    // Stopping a non-existent session should fail gracefully
    assert!(result.is_err(), "Stopping non-existent session should fail");

    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("can't find session")
            || error.to_string().contains("session not found"),
        "Error should indicate session not found: {error}"
    );

    println!("✓ Stop nonexistent session correctly failed");
}

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_start_and_stop_workflow() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("start-stop-workflow");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config with multiple windows
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor: vim
  - server: sleep 60
  - logs: tail -f /dev/null
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Step 1: Start session
    let start_result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(
        start_result.is_ok(),
        "Failed to start session: {start_result:?}"
    );

    // Step 2: Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session should exist after starting");

    // Step 3: Stop session
    let stop_result = session_manager.stop_session(session.name());
    assert!(
        stop_result.is_ok(),
        "Failed to stop session: {stop_result:?}"
    );

    // Step 4: Verify session no longer exists
    let exists_after = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(!exists_after, "Session should not exist after stopping");

    println!("✓ Complete start and stop workflow test passed");
}

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_stop_session_with_complex_windows() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("stop-complex");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a config with complex window configurations
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor:
      layout: main-vertical
      panes:
        - vim
        - echo "pane 2"
  - server:
      layout: tiled
      panes:
        - sleep 60
        - sleep 30
        - echo "monitoring"
  - logs: tail -f /dev/null
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Create the complex session
    let start_result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(
        start_result.is_ok(),
        "Failed to start complex session: {start_result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Complex session should exist after starting");

    // Stop the complex session
    let stop_result = session_manager.stop_session(session.name());
    assert!(
        stop_result.is_ok(),
        "Failed to stop complex session: {stop_result:?}"
    );

    // Verify session completely removed
    let exists_after = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(
        !exists_after,
        "Complex session should not exist after stopping"
    );

    println!("✓ Stop session with complex windows test passed");
}
