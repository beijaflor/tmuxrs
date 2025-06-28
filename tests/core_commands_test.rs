use std::path::Path;
use tempfile::TempDir;
use tmuxrs::config::Config;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_start_command_with_explicit_name() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a test config
    let config_file = config_dir.join("test-project.yml");
    let yaml_content = r#"
name: test-project
root: /tmp
windows:
  - editor: vim
  - server: echo "server starting"
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    // Clean up any existing session
    let _ = TmuxCommand::kill_session("test-project");

    // Test starting with explicit name
    let session_manager = SessionManager::new();
    let result = session_manager.start_session(Some("test-project"), Some(&config_dir));

    assert!(result.is_ok(), "Failed to start session: {:?}", result);

    // Verify session exists
    let exists = TmuxCommand::session_exists("test-project").unwrap();
    assert!(exists, "Session should exist after starting");

    // Clean up
    let _ = TmuxCommand::kill_session("test-project");
}

#[test]
fn test_start_command_with_directory_detection() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("my-rust-app");
    std::fs::create_dir(&project_dir).unwrap();

    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
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

    // Clean up any existing session
    let _ = TmuxCommand::kill_session("my-rust-app");

    // Test starting without explicit name (should detect from directory)
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_from_directory(&project_dir, Some(&config_dir));

    assert!(result.is_ok(), "Failed to start session from directory: {:?}", result);

    // Verify session exists
    let exists = TmuxCommand::session_exists("my-rust-app").unwrap();
    assert!(exists, "Session should exist after starting");

    // Clean up
    let _ = TmuxCommand::kill_session("my-rust-app");
}

#[test]
fn test_list_command() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create multiple test configs
    let configs = vec![
        ("web-app", "Web application project"),
        ("api-server", "REST API server"),
        ("data-pipeline", "Data processing pipeline"),
    ];

    for (name, _desc) in &configs {
        let config_file = config_dir.join(format!("{}.yml", name));
        let yaml_content = format!(
            r#"
name: {}
root: ~/projects/{}
windows:
  - main: echo "Starting {}"
"#,
            name, name, name
        );
        std::fs::write(&config_file, yaml_content).unwrap();
    }

    // Test listing configurations
    let session_manager = SessionManager::new();
    let result = session_manager.list_configs(Some(&config_dir));

    assert!(result.is_ok(), "Failed to list configs: {:?}", result);

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
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create and start a session first
    let config_file = config_dir.join("stop-test.yml");
    let yaml_content = r#"
name: stop-test
root: /tmp
windows:
  - main: sleep 60
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    // Clean up and create session
    let _ = TmuxCommand::kill_session("stop-test");
    TmuxCommand::new_session("stop-test", temp_dir.path()).unwrap();

    // Verify session exists
    let exists = TmuxCommand::session_exists("stop-test").unwrap();
    assert!(exists, "Session should exist before stopping");

    // Test stopping the session
    let session_manager = SessionManager::new();
    let result = session_manager.stop_session("stop-test");

    assert!(result.is_ok(), "Failed to stop session: {:?}", result);

    // Verify session no longer exists
    let exists = TmuxCommand::session_exists("stop-test").unwrap();
    assert!(!exists, "Session should not exist after stopping");
}

#[test]
fn test_attach_or_create_session() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    let config_file = config_dir.join("attach-test.yml");
    let yaml_content = r#"
name: attach-test
root: /tmp
windows:
  - editor: vim
  - terminal: bash
"#;
    std::fs::write(&config_file, yaml_content).unwrap();

    // Clean up any existing session
    let _ = TmuxCommand::kill_session("attach-test");

    let session_manager = SessionManager::new();

    // First call should create the session
    let result1 = session_manager.start_session(Some("attach-test"), Some(&config_dir));
    assert!(result1.is_ok(), "Failed to create session: {:?}", result1);

    // Verify session exists
    let exists = TmuxCommand::session_exists("attach-test").unwrap();
    assert!(exists, "Session should exist after creation");

    // Second call should detect existing session and not error
    let result2 = session_manager.start_session(Some("attach-test"), Some(&config_dir));
    assert!(result2.is_ok(), "Failed to handle existing session: {:?}", result2);

    // Clean up
    let _ = TmuxCommand::kill_session("attach-test");
}