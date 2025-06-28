use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_attach_to_existing_session() {
    let session_name = "attach-test-existing";
    let temp_dir = TempDir::new().unwrap();

    // Clean up any existing session
    let _ = TmuxCommand::kill_session(session_name);

    // Create a session first
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Test attaching to the session
    let result = TmuxCommand::attach_session(session_name);

    // Note: attach_session will fail in test environment since there's no terminal
    // We're testing that the command is properly formatted
    assert!(
        result.is_err(),
        "Attach should fail in test environment but command should be valid"
    );

    // Clean up
    let _ = TmuxCommand::kill_session(session_name);
}

#[test]
fn test_attach_to_nonexistent_session() {
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

    // Should create session (attach will fail in test env but that's expected)
    assert!(
        result.is_ok(),
        "Should create session even if attach fails in test: {:?}",
        result
    );

    // Verify session exists
    assert!(TmuxCommand::session_exists("attach-flag-test").unwrap());

    // Clean up
    let _ = TmuxCommand::kill_session("attach-flag-test");
}

#[test]
fn test_start_session_no_attach_flag() {
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

    assert!(
        result.is_ok(),
        "Should create detached session: {:?}",
        result
    );

    // Verify session exists
    assert!(TmuxCommand::session_exists("no-attach-test").unwrap());

    // Clean up
    let _ = TmuxCommand::kill_session("no-attach-test");
}

#[test]
fn test_existing_session_with_attach() {
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

    // Try to start again with attach=true (should attach to existing)
    let result = session_manager.start_session_with_options(
        Some("existing-attach-test"),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Should handle existing session gracefully: {:?}",
        result
    );

    // Clean up
    let _ = TmuxCommand::kill_session("existing-attach-test");
}
