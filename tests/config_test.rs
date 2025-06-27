use std::env;
use std::path::PathBuf;
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