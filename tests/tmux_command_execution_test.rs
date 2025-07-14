mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::error::TmuxrsError;
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_tmux_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let _session = TmuxTestSession::new("command-execution");

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
    // Automatic cleanup via Drop trait
}
