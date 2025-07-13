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
fn test_stop_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let session_name = "stop-test-existing";
    let temp_dir = TempDir::new().unwrap();

    // Create a session first
    TmuxCommand::new_session(session_name, temp_dir.path()).unwrap();

    // Verify session exists
    assert!(TmuxCommand::session_exists(session_name).unwrap());

    // Stop the session using SessionManager
    let session_manager = SessionManager::new();
    let result = session_manager.stop_session(session_name);

    assert!(result.is_ok(), "Failed to stop session: {result:?}");
    assert_eq!(result.unwrap(), format!("Stopped session '{session_name}'"));

    // Verify session no longer exists
    assert!(!TmuxCommand::session_exists(session_name).unwrap());

    println!("✓ Stop existing session test passed");
}