mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_shell_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("shell-cmd-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create tmuxrs config with initial command
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - cmd_window: echo "initial command executed"
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

    // Wait for initial command to execute
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Verify initial command was executed
    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:cmd_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("initial command executed"),
        "Initial command should execute, got: {pane_content}"
    );

    // Test additional command execution
    TmuxCommand::send_keys(session.name(), "cmd_window", "echo 'additional command'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:cmd_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("additional command"),
        "Additional commands should work, got: {pane_content}"
    );

    // Test complex command with pipes
    TmuxCommand::send_keys(
        session.name(),
        "cmd_window",
        "echo 'pipe_test' | grep 'pipe'",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:cmd_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("pipe_test"),
        "Complex shell commands should work, got: {pane_content}"
    );

    println!("✓ Shell command execution test passed");
    // Automatic cleanup via Drop trait
}
