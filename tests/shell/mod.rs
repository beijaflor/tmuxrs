use crate::common::{should_run_integration_tests, TmuxTestSession};
use std::thread;
use std::time::Duration;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

/// Tests for environment variable inheritance and shell responsiveness
#[test]
fn test_environment_inheritance() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("env-inheritance");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with environment variable usage
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Give the session time to initialize
    thread::sleep(Duration::from_millis(500));

    // Test environment variable setting and usage
    let env_cmd = TmuxCommand::send_keys(session.name(), "main", "export TEST_VAR='hello world'");
    assert!(
        env_cmd.is_ok(),
        "Failed to set environment variable: {env_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    let echo_cmd = TmuxCommand::send_keys(session.name(), "main", "echo $TEST_VAR");
    assert!(
        echo_cmd.is_ok(),
        "Failed to echo environment variable: {echo_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Capture output to verify environment variable worked
    let capture_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:main", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture_result.is_ok(),
        "Failed to capture pane: {capture_result:?}"
    );

    let output = capture_result.unwrap();
    assert!(
        output.contains("hello world"),
        "Environment variable should be accessible: {output}"
    );

    // Test shell responsiveness with a simple command
    let simple_cmd = TmuxCommand::send_keys(session.name(), "main", "echo 'shell responsive'");
    assert!(
        simple_cmd.is_ok(),
        "Failed to send simple command: {simple_cmd:?}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for interactive shell features like aliases and functions
#[test]
fn test_interactive_shell_features() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("shell-features");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config for interactive shell session
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Give the session time to initialize
    thread::sleep(Duration::from_millis(500));

    // Test shell alias functionality
    let alias_cmd = TmuxCommand::send_keys(session.name(), "main", "alias ll='ls -la'");
    assert!(alias_cmd.is_ok(), "Failed to create alias: {alias_cmd:?}");
    let enter_cmd = TmuxCommand::send_keys(session.name(), "main", "Enter");
    assert!(enter_cmd.is_ok(), "Failed to send Enter: {enter_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    let use_alias_cmd = TmuxCommand::send_keys(session.name(), "main", "ll");
    assert!(
        use_alias_cmd.is_ok(),
        "Failed to use alias: {use_alias_cmd:?}"
    );
    let enter_cmd = TmuxCommand::send_keys(session.name(), "main", "Enter");
    assert!(enter_cmd.is_ok(), "Failed to send Enter: {enter_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    // Test shell function definition and execution
    let function_cmd = TmuxCommand::send_keys(
        session.name(),
        "main",
        "myfunction() { echo 'function called with' $1; }",
    );
    assert!(
        function_cmd.is_ok(),
        "Failed to define function: {function_cmd:?}"
    );
    let enter_cmd = TmuxCommand::send_keys(session.name(), "main", "Enter");
    assert!(enter_cmd.is_ok(), "Failed to send Enter: {enter_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    let call_function_cmd = TmuxCommand::send_keys(session.name(), "main", "myfunction 'test-arg'");
    assert!(
        call_function_cmd.is_ok(),
        "Failed to call function: {call_function_cmd:?}"
    );
    let enter_cmd = TmuxCommand::send_keys(session.name(), "main", "Enter");
    assert!(enter_cmd.is_ok(), "Failed to send Enter: {enter_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    // Capture output to verify function worked
    let capture_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:main", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture_result.is_ok(),
        "Failed to capture pane: {capture_result:?}"
    );

    let output = capture_result.unwrap();
    assert!(
        output.contains("function called with test-arg"),
        "Function should work: {output}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for shell command execution from config and additional commands
#[test]
fn test_shell_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("command-execution");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with initial command
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: echo "initial command executed"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Give the session time to execute initial command
    thread::sleep(Duration::from_millis(500));

    // Test additional command execution after session creation
    let additional_cmd =
        TmuxCommand::send_keys(session.name(), "main", "echo 'additional command'");
    assert!(
        additional_cmd.is_ok(),
        "Failed to send additional command: {additional_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Test complex shell command with pipes
    let complex_cmd = TmuxCommand::send_keys(
        session.name(),
        "main",
        "echo 'test data' | grep 'test' | wc -l",
    );
    assert!(
        complex_cmd.is_ok(),
        "Failed to send complex command: {complex_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Capture output to verify commands worked
    let capture_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:main", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture_result.is_ok(),
        "Failed to capture pane: {capture_result:?}"
    );

    let output = capture_result.unwrap();
    assert!(
        output.contains("initial command executed"),
        "Initial command should execute: {output}"
    );
    assert!(
        output.contains("additional command"),
        "Additional command should execute: {output}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for shell features in split panes (currently ignored due to SessionManager limitations)
#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_shell_features_in_split_panes() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("split-panes");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with split panes
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main:
      panes:
        - echo "pane 1 initialized"
        - echo "pane 2 initialized"
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session with split panes
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to start session with split panes: {result:?}"
    );

    // Give the session time to initialize all panes
    thread::sleep(Duration::from_millis(1000));

    // Verify session exists
    let exists = TmuxCommand::session_exists(session.name()).unwrap();
    assert!(exists, "Session with split panes should exist");

    // Test that shells start correctly in split panes by sending commands to different panes
    let pane1_cmd = TmuxCommand::new()
        .arg("send-keys")
        .arg("-t")
        .arg(format!("{}:main.0", session.name()))
        .arg("echo 'pane 1 responsive'")
        .arg("Enter")
        .execute();
    assert!(
        pane1_cmd.is_ok(),
        "Failed to send command to pane 1: {pane1_cmd:?}"
    );

    let pane2_cmd = TmuxCommand::new()
        .arg("send-keys")
        .arg("-t")
        .arg(format!("{}:main.1", session.name()))
        .arg("echo 'pane 2 responsive'")
        .arg("Enter")
        .execute();
    assert!(
        pane2_cmd.is_ok(),
        "Failed to send command to pane 2: {pane2_cmd:?}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for shell initialization files and environment (currently ignored due to SessionManager limitations)
#[test]
#[ignore = "SessionManager doesn't support isolated test servers yet"]
fn test_shell_initialization_files_executed() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("shell-init");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config that uses shell initialization
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Give the session time to initialize
    thread::sleep(Duration::from_millis(500));

    // Test that standard shell variables are available (indicating proper initialization)
    let home_cmd = TmuxCommand::send_keys(session.name(), "main", "echo $HOME");
    assert!(home_cmd.is_ok(), "Failed to echo $HOME: {home_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    // Test shell responsiveness
    let responsive_cmd = TmuxCommand::send_keys(session.name(), "main", "echo 'shell initialized'");
    assert!(
        responsive_cmd.is_ok(),
        "Failed to test shell responsiveness: {responsive_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Capture output to verify shell initialization worked
    let capture_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:main", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture_result.is_ok(),
        "Failed to capture pane: {capture_result:?}"
    );

    let output = capture_result.unwrap();
    assert!(
        !output.trim().is_empty(),
        "Shell should produce output indicating initialization"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for shell state independence between windows
#[test]
fn test_shell_state_independence() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("shell-independence");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create config with multiple windows
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - window1: bash
  - window2: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(result.is_ok(), "Failed to start session: {result:?}");

    // Give the session time to initialize
    thread::sleep(Duration::from_millis(500));

    // Set environment variable in window1
    let env1_cmd = TmuxCommand::send_keys(
        session.name(),
        "window1",
        "export WINDOW_VAR='window1_value'",
    );
    assert!(
        env1_cmd.is_ok(),
        "Failed to set env var in window1: {env1_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Set different environment variable in window2
    let env2_cmd = TmuxCommand::send_keys(
        session.name(),
        "window2",
        "export WINDOW_VAR='window2_value'",
    );
    assert!(
        env2_cmd.is_ok(),
        "Failed to set env var in window2: {env2_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Test that each window maintains its own environment
    let echo1_cmd = TmuxCommand::send_keys(session.name(), "window1", "echo $WINDOW_VAR");
    assert!(
        echo1_cmd.is_ok(),
        "Failed to echo var in window1: {echo1_cmd:?}"
    );

    let echo2_cmd = TmuxCommand::send_keys(session.name(), "window2", "echo $WINDOW_VAR");
    assert!(
        echo2_cmd.is_ok(),
        "Failed to echo var in window2: {echo2_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Capture output from both windows to verify independence
    let capture1_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:window1", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture1_result.is_ok(),
        "Failed to capture window1: {capture1_result:?}"
    );

    let capture2_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:window2", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture2_result.is_ok(),
        "Failed to capture window2: {capture2_result:?}"
    );

    let output1 = capture1_result.unwrap();
    let output2 = capture2_result.unwrap();

    assert!(
        output1.contains("window1_value"),
        "Window1 should have its own env var: {output1}"
    );
    assert!(
        output2.contains("window2_value"),
        "Window2 should have its own env var: {output2}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}

/// Tests for normal operation without custom shell configuration
#[test]
fn test_no_shell_config_works_normally() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let session = TmuxTestSession::with_temp_dir("no-shell-config");
    let config_dir = session.temp_dir().unwrap().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();

    // Create minimal config without custom shell configuration
    let config_file = config_dir.join(format!("{}.yml", session.name()));
    let yaml_content = format!(
        r#"
name: {}
root: /tmp
windows:
  - main: bash
"#,
        session.name()
    );
    std::fs::write(&config_file, yaml_content).unwrap();

    // Create session
    let session_manager = SessionManager::new();
    let result = session_manager.start_session_with_options(
        Some(session.name()),
        Some(&config_dir),
        false, // attach = false
        false, // append = false
    );

    assert!(
        result.is_ok(),
        "Failed to start session without shell config: {result:?}"
    );

    // Give the session time to initialize
    thread::sleep(Duration::from_millis(500));

    // Test standard shell commands work properly
    let basic_cmd = TmuxCommand::send_keys(session.name(), "main", "echo 'basic command works'");
    assert!(
        basic_cmd.is_ok(),
        "Failed to send basic command: {basic_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    let path_cmd = TmuxCommand::send_keys(session.name(), "main", "pwd");
    assert!(path_cmd.is_ok(), "Failed to send pwd command: {path_cmd:?}");

    thread::sleep(Duration::from_millis(200));

    // Test interactive shell functionality
    let interactive_cmd =
        TmuxCommand::send_keys(session.name(), "main", "echo 'interactive works'");
    assert!(
        interactive_cmd.is_ok(),
        "Failed to send interactive command: {interactive_cmd:?}"
    );

    thread::sleep(Duration::from_millis(200));

    // Capture output to verify normal operation
    let capture_result = TmuxCommand::new()
        .arg("capture-pane")
        .arg("-t")
        .arg(format!("{}:main", session.name()))
        .arg("-p")
        .execute();
    assert!(
        capture_result.is_ok(),
        "Failed to capture pane: {capture_result:?}"
    );

    let output = capture_result.unwrap();
    assert!(
        output.contains("basic command works"),
        "Basic commands should work: {output}"
    );
    assert!(
        output.contains("interactive works"),
        "Interactive features should work: {output}"
    );

    // Clean up the session created in default tmux server
    let _ = TmuxCommand::kill_session(session.name());
}
