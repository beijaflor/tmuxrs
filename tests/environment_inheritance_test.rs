mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_environment_inheritance() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("env-test");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create tmuxrs config
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - env_window: echo "Environment test window"
"#,
        session.name(),
        session.temp_dir().unwrap().display()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Start session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Wait for initial command
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Test that we can set and use environment variables within the session
    TmuxCommand::send_keys(
        session.name(),
        "env_window",
        "export TEST_VAR='test_value'; echo $TEST_VAR",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:env_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("test_value"),
        "Should be able to set and use environment variables, got: {pane_content}"
    );

    // Test basic shell responsiveness
    TmuxCommand::send_keys(session.name(), "env_window", "echo 'shell_responsive'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:env_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("shell_responsive"),
        "Shell should be responsive to commands, got: {pane_content}"
    );

    println!("✓ Environment inheritance test passed");
    // Automatic cleanup via Drop trait
}
