use tmuxrs::tmux::TmuxCommand;

fn main() {
    println!("Tmux Integration Demo\n");

    let session_name = "tmuxrs-demo";

    // Clean up any existing session
    let _ = TmuxCommand::kill_session(session_name);

    // 1. Check if session exists (should be false)
    println!("1. Checking if session '{session_name}' exists...");
    match TmuxCommand::session_exists(session_name) {
        Ok(exists) => println!("   Session exists: {exists}"),
        Err(e) => println!("   Error: {e}"),
    }

    // 2. Create a new session
    println!("\n2. Creating session '{session_name}'...");
    let current_dir = std::env::current_dir().unwrap();
    match TmuxCommand::new_session(session_name, &current_dir) {
        Ok(_) => println!("   ✅ Session created successfully"),
        Err(e) => {
            println!("   ❌ Failed to create session: {e}");
            return;
        }
    }

    // 3. Check if session now exists (should be true)
    println!("\n3. Checking if session exists after creation...");
    match TmuxCommand::session_exists(session_name) {
        Ok(exists) => println!("   Session exists: {exists}"),
        Err(e) => println!("   Error: {e}"),
    }

    // 4. Create a new window
    println!("\n4. Creating window 'editor'...");
    match TmuxCommand::new_window(session_name, "editor", None, None) {
        Ok(_) => println!("   ✅ Window created successfully"),
        Err(e) => println!("   ❌ Failed to create window: {e}"),
    }

    // 5. Send some keys to the window
    println!("\n5. Sending command to editor window...");
    match TmuxCommand::send_keys(session_name, "editor", "echo 'Hello from tmuxrs!'") {
        Ok(_) => println!("   ✅ Keys sent successfully"),
        Err(e) => println!("   ❌ Failed to send keys: {e}"),
    }

    // 6. Create another window with a command
    println!("\n6. Creating window 'server' with a command...");
    match TmuxCommand::new_window(session_name, "server", Some("htop"), None) {
        Ok(_) => println!("   ✅ Server window created with htop"),
        Err(e) => println!("   ❌ Failed to create server window: {e}"),
    }

    // 7. List all tmux sessions
    println!("\n7. Listing all tmux sessions...");
    match TmuxCommand::new().arg("list-sessions").execute() {
        Ok(output) => println!("   Sessions:\n{output}"),
        Err(e) => println!("   ❌ Failed to list sessions: {e}"),
    }

    println!("\n🎉 Demo complete!");
    println!("You can now attach to the session with: tmux attach -t {session_name}");
    println!("Or kill it with: tmux kill-session -t {session_name}");
}
