mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_create_window() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("test-create-window-session");
    let window_name = "test-window";

    // Create session first
    session.create().unwrap();

    // Verify session exists
    assert!(session.exists().unwrap());

    // Create a new window in the session using the isolated socket
    let result = TmuxCommand::new_window_with_socket(
        session.name(),
        window_name,
        None,
        None,
        Some(session.socket_path()),
    );
    assert!(result.is_ok(), "Failed to create window: {result:?}");

    println!("✓ Window creation test passed");
    // Automatic cleanup via Drop trait
}
