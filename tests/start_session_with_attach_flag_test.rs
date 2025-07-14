use tempfile::TempDir;
use tmuxrs::session::SessionManager;

mod common;
use common::{cleanup_after_attach_test, should_run_integration_tests, TmuxTestSession};

#[test]
#[ignore = "attach tests cause hanging in Docker environment"]
fn test_start_session_with_attach_flag() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("attach-flag-test");
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create a basic config file
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor: vim
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();

    // Test starting session with attach flag
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        true,  // attach = true
        false, // append = false
    );

    // Handle both success and failure cases depending on TTY availability
    let attach_succeeded = match &result {
        Ok(msg) => {
            // Attach succeeded - valid in TTY-enabled environments
            assert!(
                msg.contains("Started and attached")
                    || msg.contains("created and attached")
                    || msg.contains("Session '"),
                "Success message should indicate session creation: {msg}"
            );
            println!("✓ Successfully created and attached to session (TTY available)");
            // Always cleanup after attach operations to prevent hanging
            cleanup_after_attach_test();
            true
        }
        Err(error) => {
            let error_msg = error.to_string();
            assert!(
                error_msg.contains("Failed to attach")
                    || error_msg.contains("but failed to attach"),
                "Error should indicate attach failure: {error_msg}"
            );
            println!("✓ Session created but attach failed as expected in non-TTY environment");
            // Cleanup after failed attach to ensure clean state
            cleanup_after_attach_test();
            false
        }
    };

    // Verify session exists (only if attach failed - successful attach might close the session)
    if !attach_succeeded {
        // When attach fails, session should still exist
        assert!(
            session.exists().unwrap(),
            "Session should exist after failed attach"
        );
    }
    // When attach succeeds, the session might already be closed - this is OK

    // Automatic cleanup via Drop trait
}
