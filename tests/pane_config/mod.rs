use crate::common::{should_run_integration_tests, TmuxTestSession};
use std::fs;
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

/// Integration tests for enhanced pane configuration (Issue #18)
/// Tests empty panes, named panes, and multiple commands

#[test]
fn test_empty_panes_create_shell_only() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("empty-panes-test");

    // Create config with empty panes
    let config_content = r#"
name: empty-panes-test
root: ~/
windows:
  - test:
      layout: main-vertical
      panes:
        - echo "pane 1"
        - ""
        - echo "pane 3"
"#;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("empty-panes-test.yml");
    fs::write(&config_file, config_content).unwrap();

    // Create session using the new pane configuration
    let manager = SessionManager::with_socket(session.socket_path());
    let result = manager.start_session_with_options(
        Some("empty-panes-test"),
        Some(temp_dir.path()),
        false, // Don't attach
        false, // Don't append
    );

    assert!(
        result.is_ok(),
        "Failed to start session with empty panes: {result:?}"
    );

    // Verify that all 3 panes exist
    let list_panes_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg("empty-panes-test:test")
        .execute();

    assert!(
        list_panes_result.is_ok(),
        "Failed to list panes: {list_panes_result:?}"
    );

    let panes_output = list_panes_result.unwrap();
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 3, "Expected 3 panes but found {pane_count}");

    // Verify that the middle pane (empty one) exists but has no command history
    // This is harder to test directly, but we can verify the panes were created
}

#[test]
fn test_named_panes_with_commands() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("named-panes-test");

    // Create config with named panes
    let config_content = r#"
name: named-panes-test
root: ~/
windows:
  - workspace:
      layout: main-vertical
      panes:
        - editor: echo "editor pane"
        - terminal: echo "terminal pane"
"#;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("named-panes-test.yml");
    fs::write(&config_file, config_content).unwrap();

    // Create session using the new pane configuration
    let manager = SessionManager::with_socket(session.socket_path());
    let result = manager.start_session_with_options(
        Some("named-panes-test"),
        Some(temp_dir.path()),
        false, // Don't attach
        false, // Don't append
    );

    assert!(
        result.is_ok(),
        "Failed to start session with named panes: {result:?}"
    );

    // Verify that 2 panes exist
    let list_panes_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg("named-panes-test:workspace")
        .execute();

    assert!(
        list_panes_result.is_ok(),
        "Failed to list panes: {list_panes_result:?}"
    );

    let panes_output = list_panes_result.unwrap();
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Expected 2 panes but found {pane_count}");
}

#[test]
fn test_multiple_commands_sequential_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("multi-commands-test");

    // Create config with multiple commands per pane
    let config_content = r#"
name: multi-commands-test
root: ~/
windows:
  - dev:
      layout: main-vertical
      panes:
        - echo "first command"
        - [echo "command 1", echo "command 2"]
"#;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("multi-commands-test.yml");
    fs::write(&config_file, config_content).unwrap();

    // Create session using the new pane configuration
    let manager = SessionManager::with_socket(session.socket_path());
    let result = manager.start_session_with_options(
        Some("multi-commands-test"),
        Some(temp_dir.path()),
        false, // Don't attach
        false, // Don't append
    );

    assert!(
        result.is_ok(),
        "Failed to start session with multiple commands: {result:?}"
    );

    // Verify that 2 panes exist
    let list_panes_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg("multi-commands-test:dev")
        .execute();

    assert!(
        list_panes_result.is_ok(),
        "Failed to list panes: {list_panes_result:?}"
    );

    let panes_output = list_panes_result.unwrap();
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Expected 2 panes but found {pane_count}");
}

#[test]
fn test_mixed_pane_configurations() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("mixed-panes-test");

    // Create config with all different pane types in one window
    let config_content = r#"
name: mixed-panes-test
root: ~/
windows:
  - mixed:
      layout: tiled
      panes:
        - echo "simple command"
        - ""
        - editor: echo "named pane"
        - [echo "multi", echo "commands"]
"#;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("mixed-panes-test.yml");
    fs::write(&config_file, config_content).unwrap();

    // Create session using the new pane configuration
    let manager = SessionManager::with_socket(session.socket_path());
    let result = manager.start_session_with_options(
        Some("mixed-panes-test"),
        Some(temp_dir.path()),
        false, // Don't attach
        false, // Don't append
    );

    assert!(
        result.is_ok(),
        "Failed to start session with mixed pane types: {result:?}"
    );

    // Verify that all 4 panes exist
    let list_panes_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg("mixed-panes-test:mixed")
        .execute();

    assert!(
        list_panes_result.is_ok(),
        "Failed to list panes: {list_panes_result:?}"
    );

    let panes_output = list_panes_result.unwrap();
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 4, "Expected 4 panes but found {pane_count}");
}
