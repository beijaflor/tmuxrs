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
fn test_create_window() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let session_name = "test-create-window-session";
    let window_name = "test-window";
    let temp_dir = TempDir::new().unwrap();

    // Create session first
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Verify session exists
    assert!(TmuxCommand::session_exists(session_name).unwrap());

    // Create a new window in the session
    let result = TmuxCommand::new_window(session_name, window_name, None, None);
    assert!(result.is_ok(), "Failed to create window: {result:?}");

    println!("✓ Window creation test passed");
}