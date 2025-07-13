mod common;

use common::should_run_integration_tests;
use tmuxrs::error::TmuxrsError;
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
fn test_tmux_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    ensure_clean_tmux();

    // Test basic tmux command execution
    let result = TmuxCommand::new().arg("list-sessions").execute();

    // Should either succeed or fail with a known error
    match result {
        Ok(_) => {
            // tmux is available and working
            println!("✓ Tmux command executed successfully");
        }
        Err(TmuxrsError::TmuxError(_)) => {
            // tmux command failed (expected if no sessions exist)
            println!("✓ Tmux command executed with expected failure (no sessions)");
        }
        Err(e) => panic!("Unexpected error type: {e}"),
    }

    println!("✓ Tmux command execution test passed");
}