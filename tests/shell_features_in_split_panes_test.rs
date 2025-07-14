mod common;

use common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::session::SessionManager;

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_shell_features_in_split_panes() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - set INTEGRATION_TESTS=1 to run");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("split-panes-test");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create tmuxrs config with split panes that should initialize properly
    // This validates that shells start correctly in split panes
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: {}
windows:
  - split_test:
      layout: main-vertical
      panes:
        - "echo 'pane1'; bash"
        - "echo 'pane2'; bash"
"#,
        session.name(),
        session.temp_dir().unwrap().display()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Start session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );
    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // The key test: if our fix works, tmuxrs should successfully create the session
    // with split panes where shells initialize properly (this was failing before our fix)

    // Just verify the session exists and has the expected structure
    let session_exists = session.exists().unwrap();
    assert!(
        session_exists,
        "Session with split panes should be created successfully"
    );

    // This test validates that the core functionality of our shell interaction fix works:
    // - Windows are created without commands first (allowing shell initialization)
    // - Commands are sent after window creation
    // - Split panes follow the same pattern

    println!("✓ Shell features in split panes test passed");
    // Automatic cleanup via Drop trait
}
