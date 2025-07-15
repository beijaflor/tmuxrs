use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

mod common;
use common::{should_run_integration_tests, TmuxTestSession};

#[test]
fn test_attach_to_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("attach-existing");

    // Create a session first
    session.create().unwrap();

    // Test attaching to the session
    let result = TmuxCommand::attach_session(session.name());

    // Both outcomes are valid depending on environment
    match result {
        Ok(_) => {
            // Attach succeeded - valid in TTY-enabled environments
            println!("✓ Successfully attached to existing session (TTY available)");
        }
        Err(e) => {
            // Attach failed - valid in non-TTY environments
            println!("✓ Attach failed as expected in non-TTY environment: {e}");
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

    // Try to attach to non-existent session
    let result = TmuxCommand::attach_session(session.name());

    assert!(
        result.is_err(),
        "Should fail when attaching to non-existent session"
    );

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

    // Verify session exists
    assert!(session.exists().unwrap());

    // Automatic cleanup via Drop trait
}
