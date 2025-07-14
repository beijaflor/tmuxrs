mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_shell_initialization_files_executed() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("shell-init-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create tmuxrs config with a bash window
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - test_window: bash
"#,
        session.name(),
        session.temp_dir().unwrap().display()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Start session using SessionManager (detached for test environment)
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false (for test environment)
        false, // append = false
    );
    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Verify session exists
    assert!(session.exists().unwrap());

    // Test that shell initialization works by checking standard variables
    TmuxCommand::send_keys(session.name(), "test_window", "echo $HOME").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Capture output to verify shell responded
    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:test_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();

    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        !pane_content.trim().is_empty() && pane_content.contains("/"),
        "HOME should be available from shell initialization, got: {pane_content}"
    );

    println!("✓ Shell initialization test passed");
    // Automatic cleanup via Drop trait
}
