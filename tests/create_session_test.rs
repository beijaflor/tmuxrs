mod common;

use common::should_run_integration_tests;
use tempfile::TempDir;
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
fn test_create_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    // Create a temporary directory for the session
    let temp_dir = TempDir::new().unwrap();
    let session_name = "test-create-session";

    // Create session
    let result = TmuxCommand::new_session(session_name, temp_dir.path());
    assert!(result.is_ok(), "Failed to create session: {result:?}");

    // Verify session exists
    let exists = TmuxCommand::session_exists(session_name).unwrap();
    assert!(exists, "Session should exist after creation");

    println!("✓ Session creation test passed");
}