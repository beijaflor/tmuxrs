use assert_cmd::Command;
use predicates::prelude::*;
use tmuxrs::config::Config;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

use crate::common::{cleanup_after_attach_test, should_run_integration_tests, TmuxTestSession};

/// Tests for CLI interface and help commands
#[test]
fn test_cli_help_displays() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A modern tmux session manager"));
}

#[test]
fn test_start_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Start a tmux session"));
}

#[test]
fn test_list_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "List available session configurations",
        ));
}

#[test]
fn test_stop_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("stop")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Stop a tmux session"));
}

#[test]
fn test_start_command_shows_attach_flags() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--attach"))
        .stdout(predicate::str::contains("--no-attach"))
        .stdout(predicate::str::contains("--append"));
}

#[test]
fn test_start_with_no_attach_flag_parsing() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("nonexistent-session")
        .arg("--no-attach")
        .assert()
        .failure() // Should fail because no config exists
        .stderr(predicate::str::contains("Configuration file not found"));
}

#[test]
fn test_start_with_append_flag_parsing() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("nonexistent-session")
        .arg("--append")
        .assert()
        .failure() // Should fail because no config exists
        .stderr(predicate::str::contains("Configuration file not found"));
}

/// Core command integration tests
#[test]
fn test_start_command_with_explicit_name() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("start-explicit");
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
  - server: echo "server starting"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Test starting with explicit name using isolated tmux server
    let session_manager = SessionManager::with_socket(session.socket_path());
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false (for test environment)
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Verify session exists in the isolated tmux server
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after starting");

    // No manual cleanup needed - TmuxTestSession's Drop trait handles it
}

#[test]
fn test_start_command_with_directory_detection() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("directory-detection");
    let project_dir = session.temp_dir().unwrap().join("my-rust-app");
    std::fs::create_dir(&project_dir).unwrap();

    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config for the detected project name
    let config_file = config_dir.join("my-rust-app.yml");
    let yaml_content = r#"
name: my-rust-app
root: ~/projects/my-rust-app
windows:
  - main: cargo run
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    // Test starting without explicit name (should detect from directory)
    let session_manager = SessionManager::with_socket(session.socket_path());
    let session_name = Config::detect_session_name(Some(&project_dir)).unwrap();
    let result = session_manager.start_session_with_options(
        Some(&session_name),
        Some(&config_dir),
        false, // attach = false (for test environment)
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to start session from directory: {result:?}"
    );

    // Verify session exists in the isolated tmux server
    let exists =
        TmuxCommand::session_exists_with_socket("my-rust-app", Some(session.socket_path()))
            .unwrap();
    assert!(exists, "Session should exist after starting");

    // No manual cleanup needed - TmuxTestSession's Drop trait handles it
}

#[test]
fn test_list_command() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("list-command");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create multiple test configs
    let configs = vec![
        ("web-app", "Web application project"),
        ("api-server", "REST API server"),
        ("data-pipeline", "Data processing pipeline"),
    ];

    for (name, _desc) in &configs {
        let config_file = config_dir.join(format!("{name}.yml"));
        let yaml_content = format!(
            r#"
name: {name}
root: ~/projects/{name}
windows:
  - main: echo "Starting {name}"
"#
        );
        std::fs::write(&config_file, yaml_content).unwrap();
    }

    // Test listing configurations
    let session_manager = SessionManager::new();
    let result = session_manager.list_configs(Some(&config_dir));

    assert!(result.is_ok(), "Failed to list configs: {result:?}");

    let configs_list = result.unwrap();
    assert_eq!(configs_list.len(), 3, "Should find 3 configurations");

    // Verify all expected configs are found
    let config_names: Vec<&str> = configs_list.iter().map(|c| c.name.as_str()).collect();
    assert!(config_names.contains(&"web-app"));
    assert!(config_names.contains(&"api-server"));
    assert!(config_names.contains(&"data-pipeline"));
}

#[test]
fn test_stop_command() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("stop-command");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create and start a session first
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

    // Create session
    session.create().unwrap();

    // Verify session exists
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist before stopping");

    // Test stopping the session using SessionManager with isolated server
    let session_manager = SessionManager::with_socket(session.socket_path());
    let result = session_manager.stop_session(session.name());

    assert!(result.is_ok(), "Failed to stop session: {result:?}");

    // Verify session no longer exists in the isolated tmux server
    let exists = session.exists().unwrap();
    assert!(!exists, "Session should not exist after stopping");
}

#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_attach_or_create_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }
    let session = TmuxTestSession::with_temp_dir("attach-create");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

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

    let session_manager = SessionManager::with_socket(session.socket_path());

    // First call should create the session (detached for test environment)
    let result1 = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false (for test environment)
        false, // append = false
    );
    assert!(result1.is_ok(), "Failed to create session: {result1:?}");

    // Verify session exists in the isolated tmux server
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after creation");

    // Second call should detect existing session and try to attach
    let result2 = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        true,  // attach = true (to test existing session attach behavior)
        false, // append = false
    );

    // Both outcomes are valid depending on environment
    match result2 {
        Ok(msg) => {
            // Attach succeeded - valid in TTY-enabled environments
            assert!(
                msg.contains("Attached to existing session"),
                "Success message should indicate attach: {msg}"
            );
            // Always cleanup after attach operations to prevent hanging
            cleanup_after_attach_test();
        }
        Err(error) => {
            // Attach failed - valid in non-TTY environments
            assert!(
                error.to_string().contains("Failed to attach"),
                "Error should indicate attach failure: {error}"
            );
            // Cleanup after failed attach to ensure clean state
            cleanup_after_attach_test();
        }
    }

    // No manual cleanup needed - TmuxTestSession's Drop trait handles it
}
