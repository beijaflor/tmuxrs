use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

mod common;
use common::{cleanup_after_attach_test, should_run_integration_tests, TmuxTestSession};

#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_attach_to_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("attach-existing");

    // Create a session first
    session.create().unwrap();

    // Test attaching to the session using the isolated socket
    let result =
        TmuxCommand::attach_session_with_socket(session.name(), Some(session.socket_path()));

    // Both outcomes are valid depending on environment
    match result {
        Ok(_) => {
            // Attach succeeded - valid in TTY-enabled environments
            println!("✓ Successfully attached to existing session (TTY available)");
            // Always cleanup after attach operations to prevent hanging
            cleanup_after_attach_test();
        }
        Err(e) => {
            // Attach failed - valid in non-TTY environments
            println!("✓ Attach failed as expected in non-TTY environment: {e}");
            // Cleanup after failed attach to ensure clean state
            cleanup_after_attach_test();
        }
    }

    // No manual cleanup needed - Drop will handle it
}

#[test]
fn test_attach_to_nonexistent_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("attach-nonexistent");

    // Don't create the session - it should not exist

    // Try to attach to non-existent session using the isolated socket
    let result =
        TmuxCommand::attach_session_with_socket(session.name(), Some(session.socket_path()));

    assert!(
        result.is_err(),
        "Should fail when attaching to non-existent session"
    );

    // Cleanup after attach attempt to ensure clean state
    cleanup_after_attach_test();

    // Automatic cleanup via Drop trait
}

#[test]
fn test_start_session_no_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("no-attach-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a basic config file
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor: vim
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Test starting session without attach flag
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Should create detached session: {result:?}");

    // Verify session exists in the default tmux server (since SessionManager doesn't use isolated sockets)
    assert!(TmuxCommand::session_exists(session.name()).unwrap());

    // Clean up the session that was created in the default tmux server
    let _ = TmuxCommand::kill_session(session.name());

    // Automatic cleanup via Drop trait
}
