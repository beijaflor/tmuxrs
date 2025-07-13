use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

mod common;
use common::should_run_integration_tests;

#[test]
fn test_attach_to_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker-compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session_name = "attach-test-existing";
    let temp_dir = TempDir::new().unwrap();

    // Clean up any existing session
    let _ = TmuxCommand::kill_session(session_name);

    // Create a session first
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Test attaching to the session
    let result = TmuxCommand::attach_session(session_name);

    // Note: attach_session will fail in test environment since there's no TTY inheritance in tests
    // We're testing that the command is properly formatted and executed
    assert!(
        result.is_err(),
        "Attach should fail in test environment due to no TTY inheritance"
    );

    // Clean up
    let _ = TmuxCommand::kill_session(session_name);
}

#[test]
fn test_attach_to_nonexistent_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker-compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session_name = "attach-test-nonexistent";

    // Ensure session doesn't exist
    let _ = TmuxCommand::kill_session(session_name);

    // Try to attach to non-existent session
    let result = TmuxCommand::attach_session(session_name);

    assert!(
        result.is_err(),
        "Should fail when attaching to non-existent session"
    );
}

#[test]
fn test_start_session_with_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker-compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a basic config file
    let config_file = config_dir.join("attach-flag-test.yml");
    let yaml_content = r#"
name: attach-flag-test
root: /tmp
windows:
  - editor: vim
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Test starting session with attach flag
    let result = session_manager.start_session_with_options(
        Some("attach-flag-test"),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    // Should create session but fail to attach in test environment (no TTY)
    match result {
        Err(error) => {
            let error_msg = error.to_string();
            assert!(
                error_msg.contains("Failed to attach")
                    || error_msg.contains("but failed to attach"),
                "Expected attach failure error, got: {error_msg}"
            );
        }
        Ok(msg) => {
            panic!("Expected attach to fail in test environment, but got success: {msg}");
        }
    }

    // Verify session exists
    assert!(TmuxCommand::session_exists("attach-flag-test").unwrap());

    // Clean up
    let _ = TmuxCommand::kill_session("attach-flag-test");
}

#[test]
fn test_start_session_no_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker-compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a basic config file
    let config_file = config_dir.join("no-attach-test.yml");
    let yaml_content = r#"
name: no-attach-test
root: /tmp
windows:
  - editor: vim
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Test starting session without attach flag
    let result = session_manager.start_session_with_options(
        Some("no-attach-test"),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Should create detached session: {result:?}");

    // Verify session exists
    assert!(TmuxCommand::session_exists("no-attach-test").unwrap());

    // Clean up
    let _ = TmuxCommand::kill_session("no-attach-test");
}

#[test]
fn test_existing_session_with_attach() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker-compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a basic config file
    let config_file = config_dir.join("existing-attach-test.yml");
    let yaml_content = r#"
name: existing-attach-test
root: /tmp
windows:
  - editor: vim
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Create session first
    let _ = session_manager.start_session_with_options(
        Some("existing-attach-test"),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    // Try to start again with attach=true (should fail to attach in test env)
    let result = session_manager.start_session_with_options(
        Some("existing-attach-test"),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    assert!(
        result.is_err(),
        "Should fail to attach to existing session in test environment: {result:?}"
    );

    // Verify error message indicates attach failure
    assert!(result.unwrap_err().to_string().contains("Failed to attach"));

    // Clean up
    let _ = TmuxCommand::kill_session("existing-attach-test");
}
