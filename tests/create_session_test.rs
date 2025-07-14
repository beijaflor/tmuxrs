mod common;

use common::{should_run_integration_tests, TmuxTestSession};

#[test]
fn test_create_session() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("create-session");

    // Create session
    let result = session.create();
    assert!(result.is_ok(), "Failed to create session: {result:?}");

    // Verify session exists
    let exists = session.exists().unwrap();
    assert!(exists, "Session should exist after creation");

    println!("✓ Session creation test passed");
    // Automatic cleanup via Drop trait
}
