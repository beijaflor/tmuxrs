mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::session::SessionManager;

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_stop_existing_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("stop-test-existing");

    // Create a session first
    session.create().unwrap();

    // Verify session exists
    assert!(session.exists().unwrap());

    // Stop the session using SessionManager
    let session_manager = SessionManager::new();
    let result = session_manager.stop_session(session.name());

    assert!(result.is_ok(), "Failed to stop session: {result:?}");
    assert_eq!(
        result.unwrap(),
        format!("Stopped session '{}'", session.name())
    );

    // Verify session no longer exists
    assert!(!session.exists().unwrap());

    println!("✓ Stop existing session test passed");
    // Automatic cleanup via Drop trait (session already stopped, but cleanup handles this)
}
