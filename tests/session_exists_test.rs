mod common;

use common::should_run_integration_tests;
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
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    let session_name = "test-nonexistent-session-12345";

    // Test that a non-existent session returns false
    let exists = TmuxCommand::session_exists(session_name).unwrap();
    assert!(!exists, "Session should not exist");

    println!("✓ Session existence check test passed");
}
