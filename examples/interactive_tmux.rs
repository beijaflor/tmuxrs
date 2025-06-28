use std::io::{self, Write};
use tmuxrs::tmux::TmuxCommand;

fn main() {
    println!("Interactive Tmux Testing");
    println!("========================");

    loop {
        println!("\nChoose an option:");
        println!("1. Check if session exists");
        println!("2. Create a session");
        println!("3. Create a window");
        println!("4. Send keys to window");
        println!("5. List sessions");
        println!("6. Kill a session");
        println!("7. Exit");

        print!("\nEnter choice (1-7): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => check_session(),
            "2" => create_session(),
            "3" => create_window(),
            "4" => send_keys(),
            "5" => list_sessions(),
            "6" => kill_session(),
            "7" => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn check_session() {
    print!("Enter session name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    match TmuxCommand::session_exists(name) {
        Ok(exists) => println!("Session '{}' exists: {}", name, exists),
        Err(e) => println!("Error: {}", e),
    }
}

fn create_session() {
    print!("Enter session name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    let current_dir = std::env::current_dir().unwrap();
    match TmuxCommand::new_session(name, &current_dir) {
        Ok(_) => println!("✅ Session '{}' created successfully", name),
        Err(e) => println!("❌ Failed to create session: {}", e),
    }
}

fn create_window() {
    print!("Enter session name: ");
    io::stdout().flush().unwrap();
    let mut session = String::new();
    io::stdin().read_line(&mut session).unwrap();
    let session = session.trim();

    print!("Enter window name: ");
    io::stdout().flush().unwrap();
    let mut window = String::new();
    io::stdin().read_line(&mut window).unwrap();
    let window = window.trim();

    match TmuxCommand::new_window(session, window, None) {
        Ok(_) => println!("✅ Window '{}' created successfully", window),
        Err(e) => println!("❌ Failed to create window: {}", e),
    }
}

fn send_keys() {
    print!("Enter session name: ");
    io::stdout().flush().unwrap();
    let mut session = String::new();
    io::stdin().read_line(&mut session).unwrap();
    let session = session.trim();

    print!("Enter window name: ");
    io::stdout().flush().unwrap();
    let mut window = String::new();
    io::stdin().read_line(&mut window).unwrap();
    let window = window.trim();

    print!("Enter command to send: ");
    io::stdout().flush().unwrap();
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim();

    match TmuxCommand::send_keys(session, window, command) {
        Ok(_) => println!("✅ Keys sent successfully"),
        Err(e) => println!("❌ Failed to send keys: {}", e),
    }
}

fn list_sessions() {
    match TmuxCommand::new().arg("list-sessions").execute() {
        Ok(output) => {
            if output.trim().is_empty() {
                println!("No tmux sessions found");
            } else {
                println!("Tmux sessions:\n{}", output);
            }
        }
        Err(e) => println!("❌ Failed to list sessions: {}", e),
    }
}

fn kill_session() {
    print!("Enter session name to kill: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    match TmuxCommand::kill_session(name) {
        Ok(_) => println!("✅ Session '{}' killed successfully", name),
        Err(e) => println!("❌ Failed to kill session: {}", e),
    }
}
