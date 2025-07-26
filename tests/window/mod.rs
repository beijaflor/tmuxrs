use crate::common::{should_run_integration_tests, TmuxTestSession};
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

/// Tests for window creation and management
#[test]
fn test_create_window() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("create-window");

    // Create the base session first
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    // Create a new window
    let window_result = session.create_window("test-window");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Verify the window was created by listing windows
    let list_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-windows")
        .arg("-t")
        .arg(session.name())
        .execute();
    assert!(
        list_result.is_ok(),
        "Failed to list windows: {list_result:?}"
    );

    let windows_output = list_result.unwrap();
    assert!(
        windows_output.contains("test-window"),
        "Window 'test-window' should appear in window list"
    );

    println!("✓ Window creation test passed");
}

/// Tests for window layout management with SessionManager (using configs)
#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_session_with_main_vertical_layout() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("main-vertical-layout");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with main-vertical layout
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor:
      layout: main-vertical
      panes:
        - vim
        - echo "Side pane"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to create session with main-vertical layout: {result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session with main-vertical layout should exist");

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());

    println!("✓ Main vertical layout test passed");
}

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_session_with_main_horizontal_layout() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("main-horizontal-layout");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with main-horizontal layout
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - editor:
      layout: main-horizontal
      panes:
        - vim
        - echo "Bottom pane"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to create session with main-horizontal layout: {result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session with main-horizontal layout should exist");

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());

    println!("✓ Main horizontal layout test passed");
}

#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_session_with_tiled_layout() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("tiled-layout");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with tiled layout and multiple panes
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - workspace:
      layout: tiled
      panes:
        - vim
        - echo "Pane 2"
        - echo "Pane 3"
        - echo "Pane 4"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to create session with tiled layout: {result:?}"
    );

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session with tiled layout should exist");

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());

    println!("✓ Tiled layout test passed");
}

/// Tests for direct window splitting operations (using isolated tmux servers)
#[test]
fn test_tmux_split_window_horizontal() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("split-horizontal");

    // Create session and window
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    let window_result = session.create_window("split-test");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Split the window horizontally (creates a pane below)
    let split_result = session.split_window_horizontal("split-test", "echo 'Bottom pane'");
    assert!(
        split_result.is_ok(),
        "Failed to split window horizontally: {split_result:?}"
    );

    // Verify the split by checking pane count
    let pane_count_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:split-test", session.name()))
        .execute();
    assert!(
        pane_count_result.is_ok(),
        "Failed to list panes: {pane_count_result:?}"
    );

    let panes_output = pane_count_result.unwrap();
    // Should have 2 panes after horizontal split
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Should have 2 panes after horizontal split");

    println!("✓ Horizontal window split test passed");
}

#[test]
fn test_tmux_split_window_vertical() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("split-vertical");

    // Create session and window
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    let window_result = session.create_window("split-test");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Split the window vertically (creates a pane to the right)
    let split_result = session.split_window_vertical("split-test", "echo 'Right pane'");
    assert!(
        split_result.is_ok(),
        "Failed to split window vertically: {split_result:?}"
    );

    // Verify the split by checking pane count
    let pane_count_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:split-test", session.name()))
        .execute();
    assert!(
        pane_count_result.is_ok(),
        "Failed to list panes: {pane_count_result:?}"
    );

    let panes_output = pane_count_result.unwrap();
    // Should have 2 panes after vertical split
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Should have 2 panes after vertical split");

    println!("✓ Vertical window split test passed");
}

#[test]
fn test_tmux_select_layout() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("select-layout");

    // Create session and window
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    let window_result = session.create_window("layout-test");
    assert!(
        window_result.is_ok(),
        "Failed to create window: {window_result:?}"
    );

    // Create multiple panes to test layouts
    let split_h_result = session.split_window_horizontal("layout-test", "echo 'Pane 2'");
    assert!(
        split_h_result.is_ok(),
        "Failed to create second pane: {split_h_result:?}"
    );

    let split_v_result = session.split_window_vertical("layout-test", "echo 'Pane 3'");
    assert!(
        split_v_result.is_ok(),
        "Failed to create third pane: {split_v_result:?}"
    );

    // Test different layouts
    let layouts = vec![
        "main-vertical",
        "main-horizontal",
        "tiled",
        "even-horizontal",
        "even-vertical",
    ];

    for layout in layouts {
        let layout_result = session.select_layout("layout-test", layout);
        assert!(
            layout_result.is_ok(),
            "Failed to select {layout} layout: {layout_result:?}"
        );
        println!("✓ Successfully applied {layout} layout");
    }

    // Verify panes still exist after layout changes
    let final_pane_count = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:layout-test", session.name()))
        .execute();
    assert!(
        final_pane_count.is_ok(),
        "Failed to list panes after layout changes: {final_pane_count:?}"
    );

    let panes_output = final_pane_count.unwrap();
    let pane_count = panes_output.lines().count();
    assert_eq!(
        pane_count, 3,
        "Should still have 3 panes after layout changes"
    );

    println!("✓ Layout selection test passed");
}
