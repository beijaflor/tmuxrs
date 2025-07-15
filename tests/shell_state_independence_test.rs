mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_shell_state_independence() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("independence-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create tmuxrs config with multiple windows
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - window1: bash
  - window2: bash
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

    // Set variable in first window
    TmuxCommand::send_keys(session.name(), "window1", "export TEST_VAR='window1_value'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Check that variable is not in second window (independent shell state)
    TmuxCommand::send_keys(session.name(), "window2", "echo $TEST_VAR").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:window2", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        !pane_content.contains("window1_value"),
        "Windows should have independent shell states, got: {pane_content}"
    );

    // Set different variable in second window
    TmuxCommand::send_keys(session.name(), "window2", "export TEST_VAR='window2_value'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Verify first window still has its value
    TmuxCommand::send_keys(session.name(), "window1", "echo $TEST_VAR").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:window1", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("window1_value"),
        "First window should maintain its state, got: {pane_content}"
    );

    println!("✓ Shell state independence test passed");
    // Automatic cleanup via Drop trait
}
