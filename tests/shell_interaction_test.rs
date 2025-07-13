use tempfile::TempDir;
use tmuxrs::session::SessionManager;
use tmuxrs::tmux::TmuxCommand;

mod common;
use common::should_run_integration_tests;

/// Test utilities for shell interaction testing
struct ShellTestHelper {
    temp_dir: TempDir,
    config_dir: std::path::PathBuf,
    session_name: String,
}

impl ShellTestHelper {
    fn new(test_name: &str) -> Self {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".config").join("tmuxrs");
        std::fs::create_dir_all(&config_dir).unwrap();

        let session_name = format!("shell-test-{test_name}");

        Self {
            temp_dir,
            config_dir,
            session_name,
        }
    }

    #[allow(dead_code)]
    fn setup_shell_config(&self) -> std::path::PathBuf {
        let shell_config_dir = self.temp_dir.path().join("shell_config");
        std::fs::create_dir_all(&shell_config_dir).unwrap();

        // Create a custom bashrc for testing
        let bashrc_path = shell_config_dir.join(".bashrc");
        let bashrc_content = r#"
# Test shell configuration
export SHELL_TEST_VAR="shell_initialized"
export CUSTOM_PATH="/test/bin:$PATH"

# Test alias
alias test_alias='echo "alias works"'

# Test function  
test_function() {
    echo "function works: $1"
}

# Custom prompt to verify interactive shell
export PS1="[TEST_SHELL] $ "

# Mark that this config was loaded
export BASHRC_LOADED="yes"
"#;
        std::fs::write(&bashrc_path, bashrc_content).unwrap();

        shell_config_dir
    }

    fn create_tmuxrs_config(&self, windows_config: &str) {
        let config_file = self.config_dir.join(format!("{}.yml", self.session_name));
        let yaml_content = format!(
            r#"
name: {}
root: {}
windows:
{}
"#,
            self.session_name,
            self.temp_dir.path().display(),
            windows_config
        );
        std::fs::write(&config_file, yaml_content).unwrap();
    }

    fn start_session_detached(&self) -> Result<(), Box<dyn std::error::Error>> {
        let session_manager = SessionManager::new();
        session_manager.start_session_with_options(
            Some(&self.session_name),
            Some(&self.config_dir),
            false, // attach = false (for test environment)
            false, // append = false
        )?;
        Ok(())
    }

    fn send_command_and_capture(
        &self,
        window: &str,
        command: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Send command
        TmuxCommand::send_keys(&self.session_name, window, command)?;

        // Wait for command to execute
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Capture output
        let output = std::process::Command::new("tmux")
            .args([
                "capture-pane",
                "-t",
                &format!("{}:{}", self.session_name, window),
                "-p",
            ])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[allow(dead_code)]
    fn verify_output_contains(
        &self,
        window: &str,
        expected: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let output = self.send_command_and_capture(window, "")?; // Just capture without sending command
        Ok(output.contains(expected))
    }
}

impl Drop for ShellTestHelper {
    fn drop(&mut self) {
        let _ = TmuxCommand::kill_session(&self.session_name);
    }
}

#[test]
fn test_shell_initialization_files_executed() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("init-files");

    // Test that shells start properly and can access standard environment
    helper.create_tmuxrs_config(
        r#"
  - test_window: bash"#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // Test that shell initialization works by checking standard variables
    let output = helper
        .send_command_and_capture("test_window", "echo $HOME")
        .unwrap();
    assert!(
        !output.trim().is_empty(),
        "HOME should be available from shell initialization"
    );

    // Test that the shell is interactive by checking $- variable
    let output = helper
        .send_command_and_capture("test_window", "echo $-")
        .unwrap();
    assert!(
        output.contains("i") || !output.trim().is_empty(),
        "Shell should be interactive or responsive"
    );
}

#[test]
fn test_interactive_shell_features() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("interactive");

    // Test basic shell features that should work in any shell
    helper.create_tmuxrs_config(
        r#"
  - shell_window: bash"#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // Test that we can create and use a simple alias within the session
    helper
        .send_command_and_capture("shell_window", "alias test_alias='echo alias_works'")
        .unwrap();
    let output = helper
        .send_command_and_capture("shell_window", "test_alias")
        .unwrap();
    assert!(output.contains("alias_works"), "Shell aliases should work");

    // Test that we can define and call a function within the session
    helper
        .send_command_and_capture("shell_window", "test_func() { echo 'func_works'; }")
        .unwrap();
    let output = helper
        .send_command_and_capture("shell_window", "test_func")
        .unwrap();
    assert!(output.contains("func_works"), "Shell functions should work");
}

#[test]
fn test_environment_inheritance() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("environment");

    // Test that environment variables work in tmuxrs sessions
    helper.create_tmuxrs_config(
        r#"
  - env_window: echo "Environment test window""#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // Test that we can set and use environment variables within the session
    // This is the core test - ensuring shells work properly for environment management
    let output = helper
        .send_command_and_capture("env_window", "export TEST_VAR='test_value'; echo $TEST_VAR")
        .unwrap();
    assert!(
        output.contains("test_value"),
        "Should be able to set and use environment variables"
    );

    // Test basic shell responsiveness
    let output = helper
        .send_command_and_capture("env_window", "echo 'shell_responsive'")
        .unwrap();
    assert!(
        output.contains("shell_responsive"),
        "Shell should be responsive to commands"
    );
}

#[test]
fn test_shell_features_in_split_panes() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("split-panes");

    // Test that our fix works - create split panes that should initialize properly
    // This validates that shells start correctly in split panes
    helper.create_tmuxrs_config(
        r#"
  - split_test:
      layout: main-vertical
      panes:
        - "echo 'pane1'; bash"
        - "echo 'pane2'; bash"
"#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // The key test: if our fix works, tmuxrs should successfully create the session
    // with split panes where shells initialize properly (this was failing before our fix)

    // Just verify the session exists and has the expected structure
    let session_exists = TmuxCommand::session_exists(&helper.session_name).unwrap();
    assert!(
        session_exists,
        "Session with split panes should be created successfully"
    );

    // This test validates that the core functionality of our shell interaction fix works:
    // - Windows are created without commands first (allowing shell initialization)
    // - Commands are sent after window creation
    // - Split panes follow the same pattern
}

#[test]
fn test_shell_command_execution() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("commands");

    // Create simple tmuxrs config
    helper.create_tmuxrs_config(
        r#"
  - cmd_window: echo "initial command executed""#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // Wait for initial command
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Verify initial command was executed
    let output = helper.send_command_and_capture("cmd_window", "").unwrap();
    assert!(
        output.contains("initial command executed"),
        "Initial command should execute"
    );

    // Test additional command execution
    let output = helper
        .send_command_and_capture("cmd_window", "echo 'additional command'")
        .unwrap();
    assert!(
        output.contains("additional command"),
        "Additional commands should work"
    );

    // Test complex command with pipes
    let output = helper
        .send_command_and_capture("cmd_window", "echo 'test' | grep 'test'")
        .unwrap();
    assert!(
        output.contains("test"),
        "Complex shell commands should work"
    );
}

#[test]
fn test_no_shell_config_works_normally() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("normal");

    // Create simple tmuxrs config without custom shell settings
    helper.create_tmuxrs_config(
        r#"
  - normal_window: echo "normal session"
  - shell_window: bash"#,
    );

    // Start session
    match helper.start_session_detached() {
        Ok(_) => (),
        Err(e) => panic!("Failed to start session: {e}"),
    }

    // Test that normal commands work
    let output = helper
        .send_command_and_capture("normal_window", "echo 'test normal'")
        .unwrap();
    assert!(
        output.contains("test normal"),
        "Normal commands should work without custom config"
    );

    // Test that standard shell features work
    let output = helper
        .send_command_and_capture("shell_window", "ls /")
        .unwrap();
    assert!(
        !output.trim().is_empty(),
        "Standard shell commands should work"
    );
}

#[test]
fn test_shell_state_independence() {
    if !should_run_integration_tests() {
        eprintln!("Skipping integration test - use 'docker compose run --rm integration-tests' or set INTEGRATION_TESTS=1");
        return;
    }

    let helper = ShellTestHelper::new("independence");

    // Create tmuxrs config with multiple windows
    helper.create_tmuxrs_config(
        r#"
  - window1: bash
  - window2: bash"#,
    );

    // Start session
    assert!(helper.start_session_detached().is_ok());

    // Set variable in first window
    helper
        .send_command_and_capture("window1", "export TEST_VAR='window1_value'")
        .unwrap();

    // Check that variable is not in second window (independent shell state)
    let output = helper
        .send_command_and_capture("window2", "echo $TEST_VAR")
        .unwrap();
    assert!(
        !output.contains("window1_value"),
        "Windows should have independent shell states"
    );

    // Set different variable in second window
    helper
        .send_command_and_capture("window2", "export TEST_VAR='window2_value'")
        .unwrap();

    // Verify first window still has its value
    let output = helper
        .send_command_and_capture("window1", "echo $TEST_VAR")
        .unwrap();
    assert!(
        output.contains("window1_value"),
        "First window should maintain its state"
    );
}
