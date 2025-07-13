use tempfile::TempDir;
use tmuxrs::session::SessionManager;

mod common;
use common::{should_run_integration_tests, TmuxTestSession};

#[test]
fn test_existing_session_with_attach() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("existing-attach-test");
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

    // Create session first
    let _ = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    // Try to start again with attach=true (should fail to attach in test env)
    let result = session_manager.start_session_with_options(
        Some(session.name()),
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

    // Automatic cleanup via Drop trait
}
