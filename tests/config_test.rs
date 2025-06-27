use std::env;
use tempfile::TempDir;
use tmuxrs::config::Config;
use tmuxrs::error::TmuxrsError;

#[test]
fn test_detect_session_name_from_directory() {
    // Test directory basename detection
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().join("my-awesome-project");
    std::fs::create_dir(&project_path).unwrap();
    
    env::set_current_dir(&project_path).unwrap();
    
    let session_name = Config::detect_session_name().unwrap();
    assert_eq!(session_name, "my-awesome-project");
}

#[test]
fn test_get_config_file_path() {
    let config_path = Config::get_config_file_path("test-session").unwrap();
    
    // Should be ~/.config/tmuxrs/test-session.yml
    assert!(config_path.to_string_lossy().contains(".config/tmuxrs"));
    assert!(config_path.to_string_lossy().ends_with("test-session.yml"));
}

#[test]
fn test_load_config_file_not_found() {
    let result = Config::load("nonexistent-session");
    
    match result {
        Err(TmuxrsError::ConfigNotFound(_)) => {
            // Expected error
        }
        _ => panic!("Expected ConfigNotFound error"),
    }
}

#[test]
fn test_parse_yaml_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("test.yml");
    
    let yaml_content = r#"
name: test-session
root: ~/projects/test
windows:
  - editor: vim
  - server: rails server
"#;
    
    std::fs::write(&config_file, yaml_content).unwrap();
    
    let config = Config::parse_file(&config_file).unwrap();
    assert_eq!(config.name, "test-session");
    assert_eq!(config.root, Some("~/projects/test".to_string()));
    assert_eq!(config.windows.len(), 2);
}

#[test]
fn test_configuration_discovery_integration() {
    // This test verifies the complete configuration discovery flow:
    // 1. Change to a directory
    // 2. Detect session name from directory
    // 3. Resolve config path
    // 4. Load config if it exists
    
    let temp_dir = TempDir::new().unwrap();
    
    // Create a project directory
    let project_dir = temp_dir.path().join("my-rust-project");
    std::fs::create_dir(&project_dir).unwrap();
    
    // Create a mock config directory structure
    let config_dir = temp_dir.path().join(".config").join("tmuxrs");
    std::fs::create_dir_all(&config_dir).unwrap();
    
    // Create config file for the project
    let config_file = config_dir.join("my-rust-project.yml");
    let yaml_content = r#"
name: my-rust-project
root: ~/code/my-rust-project
windows:
  - editor: vim src/main.rs
  - server: cargo run
  - git: lazygit
"#;
    std::fs::write(&config_file, yaml_content).unwrap();
    
    // Change to project directory
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(&project_dir).unwrap();
    
    // Test the discovery flow
    let detected_name = Config::detect_session_name().unwrap();
    assert_eq!(detected_name, "my-rust-project");
    
    // In real usage, we'd use dirs::home_dir(), but for testing we'll parse directly
    let loaded_config = Config::parse_file(&config_file).unwrap();
    assert_eq!(loaded_config.name, "my-rust-project");
    assert_eq!(loaded_config.windows.len(), 3);
    
    // Restore original directory
    env::set_current_dir(&original_dir).unwrap();
}

#[test]
fn test_detect_session_name_different_directories() {
    let original_dir = env::current_dir().unwrap();
    let temp_dir = TempDir::new().unwrap();
    
    // Test various directory names
    let test_cases = vec![
        "web-app",
        "my-project",
        "tmuxrs",
        "backend-api",
        "123-numbers",
    ];
    
    for dir_name in test_cases {
        let test_dir = temp_dir.path().join(dir_name);
        std::fs::create_dir(&test_dir).unwrap();
        env::set_current_dir(&test_dir).unwrap();
        
        let detected = Config::detect_session_name().unwrap();
        assert_eq!(detected, dir_name, "Failed to detect session name for directory: {}", dir_name);
    }
    
    // Restore original directory
    env::set_current_dir(&original_dir).unwrap();
}