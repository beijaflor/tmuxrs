mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::tmux::TmuxCommand;

#[test]
fn test_session_exists_check() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("test-exists-check");

    // Test that session doesn't exist before creation
    assert!(
        !session.exists().unwrap(),
        "Session should not exist before creation"
    );

    // Create the session
    session.create().unwrap();

    // Test that session exists after creation
    assert!(
        session.exists().unwrap(),
        "Session should exist after creation"
    );

    // Test with a completely different session name that doesn't exist
    let nonexistent_exists = TmuxCommand::session_exists("test-nonexistent-session-12345").unwrap();
    assert!(
        !nonexistent_exists,
        "Non-existent session should return false"
    );

    println!("✓ Session existence check test passed");
    // Automatic cleanup via Drop trait
}
