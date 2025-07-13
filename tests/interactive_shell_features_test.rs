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
fn test_interactive_shell_features() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    let session_name = "interactive-test";

    // Create tmuxrs config with bash window for interactive features
    let config_file = config_dir.join(format!("{session_name}.yml"));
    let yaml_content = format!(
        r#"
name: {session_name}
root: {}
windows:
  - shell_window: bash
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

    // Test that we can create and use a simple alias within the session
    TmuxCommand::send_keys(
        session_name,
        "shell_window",
        "alias test_alias='echo alias_works'",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    TmuxCommand::send_keys(session_name, "shell_window", "test_alias").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{session_name}:shell_window"),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("alias_works"),
        "Shell aliases should work, got: {pane_content}"
    );

    // Test that we can define and call a function within the session
    TmuxCommand::send_keys(
        session_name,
        "shell_window",
        "test_func() { echo 'func_works'; }",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    TmuxCommand::send_keys(session_name, "shell_window", "test_func").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));

    let output = std::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &format!("{session_name}:shell_window"),
            "-p",
        ])
        .output()
        .unwrap();
    let pane_content = String::from_utf8_lossy(&output.stdout);
    assert!(
        pane_content.contains("func_works"),
        "Shell functions should work, got: {pane_content}"
    );

    println!("✓ Interactive shell features test passed");
}
