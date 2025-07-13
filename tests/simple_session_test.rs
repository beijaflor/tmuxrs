mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use std::process::Command;

#[test]
fn test_basic_session_creation() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    // Create a test session using the helper
    let session = TmuxTestSession::new("basic");

    // Actually create the tmux session
    session.create().expect("Failed to create tmux session");

    // Get the session name for verification
    let session_name = session.name();

    // Verify the session exists by listing sessions
    let output = Command::new("tmux")
        .arg("list-sessions")
        .arg("-F")
        .arg("#{session_name}")
        .output()
        .expect("Failed to list tmux sessions");

    let sessions = String::from_utf8(output.stdout).expect("Invalid UTF-8 in tmux output");

    // Check that our session is in the list
    assert!(
        sessions.lines().any(|line| line == session_name),
        "Session '{session_name}' not found in tmux list-sessions output: {sessions}"
    );

    // Verify session exists using has-session
    let has_session = Command::new("tmux")
        .arg("has-session")
        .arg("-t")
        .arg(session_name)
        .status()
        .expect("Failed to check if session exists");

    assert!(
        has_session.success(),
        "tmux has-session failed for '{session_name}'"
    );

    println!("✓ Basic session creation test passed");
}
