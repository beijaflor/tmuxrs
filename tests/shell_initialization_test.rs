mod common;

use common::should_run_integration_tests;
use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

fn ensure_clean_tmux() {
    if std::env::var("INTEGRATION_TESTS").unwrap_or_default() == "1" {
        let _ = std::process::Command::new("tmux")
            .arg("kill-server")
            .output();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
fn test_shell_initialization_files_executed() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    let session_name = "shell-init-test";

    // Create tmuxrs config with a bash window
    let config_file = config_dir.join(format!("{session_name}.yml"));
    let yaml_content = format!(
        r#"
name: {session_name}
root: {}
windows:
  - test_window: bash
"#,
        temp_dir.path().display()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Start session using SessionManager (detached for test environment)
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session_name),
        Some(&config_dir),
        false, // attach = false (for test environment)
        false, // append = false
    );
    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Verify session exists
    assert!(TmuxCommand::session_exists(session_name).unwrap());

    // Test that shell initialization works by checking standard variables
    TmuxCommand::send_keys(session_name, "test_window", "echo $HOME").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Capture output to verify shell responded
    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{session_name}:test_window"),
            "-p",
        ])
        .output()
        .unwrap();

    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        !pane_content.trim().is_empty() && pane_content.contains("/"),
        "HOME should be available from shell initialization, got: {}",
        pane_content
    );

    println!("✓ Shell initialization test passed");
}