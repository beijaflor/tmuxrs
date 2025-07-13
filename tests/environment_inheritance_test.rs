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
fn test_environment_inheritance() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    let session_name = "env-test";

    // Create tmuxrs config
    let config_file = config_dir.join(format!("{session_name}.yml"));
    let yaml_content = format!(
        r#"
name: {session_name}
root: {}
windows:
  - env_window: echo "Environment test window"
"#,
        temp_dir.path().display()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Start session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session_name),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Wait for initial command
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Test that we can set and use environment variables within the session
    TmuxCommand::send_keys(
        session_name,
        "env_window",
        "export TEST_VAR='test_value'; echo $TEST_VAR",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{session_name}:env_window"),
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
    TmuxCommand::send_keys(session_name, "env_window", "echo 'shell_responsive'").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{session_name}:env_window"),
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
}
