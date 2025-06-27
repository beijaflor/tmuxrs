use tmuxrs::config::Config;

fn main() {
    println!("Configuration Discovery Demo\n");
    
    // 1. Test directory basename detection
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    match Config::detect_session_name(None) {
        Ok(name) => println!("Detected session name: {}", name),
        Err(e) => println!("Error detecting session name: {}", e),
    }
    
    // 2. Test config file path resolution
    println!("\nConfig file paths:");
    for session in &["tmuxrs", "my-project", "test-session"] {
        match Config::get_config_file_path(session) {
            Ok(path) => println!("  {} -> {}", session, path.display()),
            Err(e) => println!("  {} -> Error: {}", session, e),
        }
    }
    
    // 3. Test loading a config based on detected session name
    println!("\nTrying to load config based on current directory:");
    match Config::detect_session_name(None) {
        Ok(session_name) => {
            println!("Attempting to load config for session: {}", session_name);
            match Config::load(&session_name) {
                Ok(config) => println!("Loaded config: {:?}", config),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error detecting session name: {}", e),
    }
    
    // 4. Demonstrate detection with explicit path
    println!("\nDetecting session name from specific paths:");
    for path in &["/tmp", "/Users", "/"] {
        match std::path::Path::new(path).try_exists() {
            Ok(true) => {
                match Config::detect_session_name(Some(std::path::Path::new(path))) {
                    Ok(name) => println!("  {} -> {}", path, name),
                    Err(e) => println!("  {} -> Error: {}", path, e),
                }
            }
            _ => println!("  {} -> Path doesn't exist", path),
        }
    }
}