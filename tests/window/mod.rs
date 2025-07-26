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
}

/// Tests for window layout management with SessionManager (using configs)
#[test]
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

    let session_manager = SessionManager::with_socket(session.socket_path());
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

    // Verify session exists on isolated server
    let exists = session.exists().unwrap();
    assert!(exists, "Session with main-vertical layout should exist");

    // Session cleanup happens automatically via TmuxTestSession::Drop
}

#[test]
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

    let session_manager = SessionManager::with_socket(session.socket_path());
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

    // Verify session exists on isolated server
    let exists = session.exists().unwrap();
    assert!(exists, "Session with main-horizontal layout should exist");

    // Session cleanup happens automatically via TmuxTestSession::Drop
}

#[test]
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

    let session_manager = SessionManager::with_socket(session.socket_path());
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

    // Verify session exists on isolated server
    let exists = session.exists().unwrap();
    assert!(exists, "Session with tiled layout should exist");

    // Session cleanup happens automatically via TmuxTestSession::Drop
}

/// Tests for direct window splitting operations (using isolated tmux servers)
#[test]
fn test_tmux_split_window_horizontal() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("split-horizontal");

    // Create session (which creates default window)
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    // List windows to see what was created
    let list_windows = TmuxCommand::with_socket(session.socket_path())
        .arg("list-windows")
        .arg("-t")
        .arg(session.name())
        .execute();
    assert!(list_windows.is_ok(), "Failed to list windows");
    let windows = list_windows.unwrap();

    // Extract the first window index from the output
    let window_index = windows
        .lines()
        .next()
        .and_then(|line| line.split(':').next())
        .unwrap_or("0");

    // Split the default window horizontally (creates a pane below)
    let split_result = session.split_window_horizontal(window_index, "");
    assert!(
        split_result.is_ok(),
        "Failed to split window horizontally: {split_result:?}"
    );

    // Verify the split by checking pane count
    let pane_count_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:{}", session.name(), window_index))
        .execute();
    assert!(
        pane_count_result.is_ok(),
        "Failed to list panes: {pane_count_result:?}"
    );

    let panes_output = pane_count_result.unwrap();
    // Should have 2 panes after horizontal split
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Should have 2 panes after horizontal split");
}

#[test]
fn test_tmux_split_window_vertical() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("split-vertical");

    // Create session (which creates default window)
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    // List windows to see what was created
    let list_windows = TmuxCommand::with_socket(session.socket_path())
        .arg("list-windows")
        .arg("-t")
        .arg(session.name())
        .execute();
    assert!(list_windows.is_ok(), "Failed to list windows");
    let windows = list_windows.unwrap();

    // Extract the first window index from the output
    let window_index = windows
        .lines()
        .next()
        .and_then(|line| line.split(':').next())
        .unwrap_or("0");

    // Split the default window vertically (creates a pane to the right)
    let split_result = session.split_window_vertical(window_index, "");
    assert!(
        split_result.is_ok(),
        "Failed to split window vertically: {split_result:?}"
    );

    // Verify the split by checking pane count
    let pane_count_result = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:{}", session.name(), window_index))
        .execute();
    assert!(
        pane_count_result.is_ok(),
        "Failed to list panes: {pane_count_result:?}"
    );

    let panes_output = pane_count_result.unwrap();
    // Should have 2 panes after vertical split
    let pane_count = panes_output.lines().count();
    assert_eq!(pane_count, 2, "Should have 2 panes after vertical split");
}

#[test]
fn test_tmux_select_layout() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::new("select-layout");

    // Create session (which creates default window)
    let session_result = session.create();
    assert!(
        session_result.is_ok(),
        "Failed to create session: {session_result:?}"
    );

    // List windows to see what was created
    let list_windows = TmuxCommand::with_socket(session.socket_path())
        .arg("list-windows")
        .arg("-t")
        .arg(session.name())
        .execute();
    assert!(list_windows.is_ok(), "Failed to list windows");
    let windows = list_windows.unwrap();

    // Extract the first window index from the output
    let window_index = windows
        .lines()
        .next()
        .and_then(|line| line.split(':').next())
        .unwrap_or("0");

    // Create multiple panes to test layouts
    let split_h_result = session.split_window_horizontal(window_index, "");
    assert!(
        split_h_result.is_ok(),
        "Failed to create second pane: {split_h_result:?}"
    );

    // Check pane count after first split
    let pane_check1 = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:{}", session.name(), window_index))
        .execute();
    assert!(
        pane_check1.is_ok(),
        "Failed to list panes after first split"
    );
    let pane_count1 = pane_check1.unwrap().lines().count();
    assert_eq!(pane_count1, 2, "Should have 2 panes after first split");

    let split_v_result = session.split_window_vertical(window_index, "");
    assert!(
        split_v_result.is_ok(),
        "Failed to create third pane: {split_v_result:?}"
    );

    // Check pane count after second split
    let pane_check2 = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:{}", session.name(), window_index))
        .execute();
    assert!(
        pane_check2.is_ok(),
        "Failed to list panes after second split"
    );
    let pane_count2 = pane_check2.unwrap().lines().count();
    assert_eq!(pane_count2, 3, "Should have 3 panes after second split");

    // Test different layouts
    let layouts = vec![
        "main-vertical",
        "main-horizontal",
        "tiled",
        "even-horizontal",
        "even-vertical",
    ];

    for layout in layouts {
        let layout_result = session.select_layout(window_index, layout);
        assert!(
            layout_result.is_ok(),
            "Failed to select {layout} layout: {layout_result:?}"
        );
    }

    // Verify panes still exist after layout changes
    let final_pane_count = TmuxCommand::with_socket(session.socket_path())
        .arg("list-panes")
        .arg("-t")
        .arg(format!("{}:{}", session.name(), window_index))
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
}
