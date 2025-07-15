mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_no_shell_config_works_normally() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("normal-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create simple tmuxrs config without custom shell settings
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - normal_window: echo "normal session"
  - shell_window: bash
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

    // Test that normal commands work
    TmuxCommand::send_keys(session.name(), "normal_window", "echo 'test normal'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:normal_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("test normal"),
        "Normal commands should work without custom config, got: {pane_content}"
    );

    // Test that standard shell features work
    TmuxCommand::send_keys(session.name(), "shell_window", "ls /").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{}:shell_window", session.name()),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        !pane_content.trim().is_empty(),
        "Standard shell commands should work, got: {pane_content}"
    );

    println!("✓ No shell config works normally test passed");
    // Automatic cleanup via Drop trait
}
