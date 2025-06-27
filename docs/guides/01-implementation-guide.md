# tmuxrs Implementation Strategy

*Practical implementation of the [System Architecture](../design/02-system-architecture.md). Refer to [Feature Decisions](../design/01-feature-decisions.md) for context on design choices.*

## Implementation Phases

### Phase 0: Project Setup (30 minutes)
1. Initialize Rust project with cargo
2. Set up basic CLI structure
3. Add dependencies
4. Create module skeleton
5. Set up CI/CD (optional for MVP)

### Phase 1: Foundation (2-3 hours)
1. CLI argument parsing with clap
2. Configuration file discovery logic
3. YAML parsing and validation
4. Basic error types

### Phase 2: Tmux Integration (2-3 hours)
1. Tmux command abstraction
2. Command execution wrapper
3. Session existence checking
4. Basic session operations

### Phase 3: Core Features (3-4 hours)
1. Session creation from config
2. Window creation with layouts
3. Attach logic (smart detection)
4. List and stop commands

### Phase 4: Polish & Testing (2-3 hours)
1. Error message improvements
2. Integration tests
3. Manual testing
4. Documentation

**Total Estimated Time: 10-14 hours for MVP**

## Project Setup Guide

### 1. Initialize Project
```bash
cargo new tmuxrs
cd tmuxrs

# Create module structure
mkdir -p src/{config,session,tmux}
touch src/{cli,error}.rs
touch src/{config,session,tmux}/mod.rs
```

### 2. Cargo.toml Setup
```toml
[package]
name = "tmuxrs"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "A modern tmux session manager"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/tmuxrs"

[dependencies]
clap = { version = "4.5", features = ["derive", "env"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "1.0"
dirs = "5.0"

[dev-dependencies]
tempfile = "3.10"
assert_cmd = "2.0"
predicates = "3.1"
```

### 3. Initial main.rs
```rust
mod cli;
mod config;
mod error;
mod session;
mod tmux;

use clap::Parser;
use error::Result;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    
    match args.command {
        cli::Command::Start { name } => {
            // TODO: Implement start command
            println!("Starting session: {:?}", name);
        }
        cli::Command::List => {
            // TODO: Implement list command
            println!("Listing sessions");
        }
        cli::Command::Stop { name } => {
            // TODO: Implement stop command
            println!("Stopping session: {}", name);
        }
    }
    
    Ok(())
}
```

## Implementation Order

### 1. Start with CLI (src/cli.rs)
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start a tmux session
    Start {
        /// Session name (optional, detects from directory if not provided)
        name: Option<String>,
    },
    /// List available session configurations
    List,
    /// Stop a tmux session
    Stop {
        /// Session name to stop
        name: String,
    },
}
```

### 2. Then Error Types (src/error.rs)
```rust
use thiserror::Error;

pub type Result<T> = std::result::Result<T, TmuxrsError>;

#[derive(Debug, Error)]
pub enum TmuxrsError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),
    
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    
    #[error("tmux command failed: {0}")]
    TmuxError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### 3. Config Discovery (src/config/mod.rs)
```rust
use std::path::{Path, PathBuf};
use crate::error::Result;

pub fn find_config(name: Option<&str>) -> Result<PathBuf> {
    match name {
        Some(name) => find_named_config(name),
        None => find_directory_config(),
    }
}

fn find_directory_config() -> Result<PathBuf> {
    // 1. Check ./.tmuxinator.yml
    // 2. Get directory basename
    // 3. Check ~/.config/tmuxrs/{basename}.yml
    todo!()
}

fn find_named_config(name: &str) -> Result<PathBuf> {
    // Check ~/.config/tmuxrs/{name}.yml
    todo!()
}
```

### 4. Tmux Commands (src/tmux/mod.rs)
```rust
use std::process::Command;
use crate::error::{Result, TmuxrsError};

pub fn has_session(name: &str) -> Result<bool> {
    let output = Command::new("tmux")
        .args(["has-session", "-t", name])
        .output()?;
    
    Ok(output.status.success())
}

pub fn attach_session(name: &str) -> Result<()> {
    let status = Command::new("tmux")
        .args(["attach-session", "-t", name])
        .status()?;
    
    if !status.success() {
        return Err(TmuxrsError::TmuxError(
            format!("Failed to attach to session: {}", name)
        ));
    }
    
    Ok(())
}
```

## Configuration Format Examples

### Basic tmuxinator-compatible format:
```yaml
name: myproject
root: ~/projects/myproject

windows:
  - editor:
      layout: main-vertical
      panes:
        - nvim
  - server:
      panes:
        - cargo run
  - logs:
      panes:
        - tail -f log/dev.log
```

### MVP simplified format:
```yaml
name: myproject
root: ~/projects/myproject

windows:
  - editor: vim           # Simple format for MVP
  - server: cargo run     # Single pane per window
  - logs: tail -f log/dev.log
```

## Testing Strategy

### 1. Unit Tests
Each module should have tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_discovery() {
        // Test directory basename extraction
        // Test config file path construction
    }
    
    #[test]
    fn test_yaml_parsing() {
        let yaml = r#"
name: test
windows:
  - editor: vim
  - server: npm start
"#;
        // Test parsing and validation
    }
}
```

### 2. Integration Tests
Create `tests/integration.rs`:

```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_start_without_args() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No configuration found"));
}

#[test]
fn test_list_command() {
    // Create test config directory
    // Run list command
    // Assert output contains config
}
```

### 3. Manual Testing Script
Create `test.sh`:

```bash
#!/bin/bash
set -e

echo "=== tmuxrs Manual Test Suite ==="

# Test 1: Create test config
mkdir -p ~/.config/tmuxrs
cat > ~/.config/tmuxrs/test.yml << EOF
name: test
windows:
  - editor: echo "vim"
  - server: echo "server"
EOF

# Test 2: Start session
echo "Test: Starting session..."
cargo run -- start test

# Test 3: List sessions
echo "Test: Listing configs..."
cargo run -- list

# Test 4: Stop session
echo "Test: Stopping session..."
cargo run -- stop test

echo "=== All tests passed! ==="
```

## Development Workflow

### 1. Incremental Implementation
- Start with `tmuxrs list` (easiest command)
- Then `tmuxrs start <name>` (explicit name)
- Then `tmuxrs start` (directory detection)
- Finally `tmuxrs stop`

### 2. Test-Driven Development
```bash
# Write failing test
cargo test test_config_discovery -- --nocapture

# Implement until test passes
cargo watch -x test

# Manual verification
cargo run -- list
```

### 3. Quick Iteration Cycle
```bash
# Use cargo-watch for auto-rebuild
cargo install cargo-watch
cargo watch -x 'run -- start test'

# Check tmux state
tmux ls
tmux attach -t test
```

## Common Implementation Patterns

### 1. Command Execution Pattern
```rust
fn execute_tmux_command(args: &[&str]) -> Result<String> {
    let output = Command::new("tmux")
        .args(args)
        .output()
        .map_err(|e| TmuxrsError::TmuxError(
            format!("Failed to execute tmux: {}", e)
        ))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TmuxrsError::TmuxError(stderr.to_string()));
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### 2. Config Path Resolution
```rust
fn get_config_dir() -> Result<PathBuf> {
    dirs::config_dir()
        .ok_or_else(|| TmuxrsError::ConfigNotFound(
            "Could not determine config directory".to_string()
        ))
        .map(|p| p.join("tmuxrs"))
}
```

### 3. Session Name Sanitization
```rust
fn sanitize_session_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}
```

## Debugging Tips

### 1. Tmux Command Debugging
```rust
// Add debug flag to see actual commands
if std::env::var("TMUXRS_DEBUG").is_ok() {
    eprintln!("Executing: tmux {}", args.join(" "));
}
```

### 2. Config Discovery Debugging
```rust
// Print searched paths
if std::env::var("TMUXRS_DEBUG").is_ok() {
    eprintln!("Searching for config in: {:?}", path);
}
```

### 3. Test Tmux Commands Manually
```bash
# Test commands tmuxrs will use
tmux has-session -t myproject
tmux new-session -d -s myproject -c ~/projects/myproject
tmux new-window -t myproject -n server
tmux send-keys -t myproject:server 'npm start' Enter
```

## Performance Considerations

### 1. Lazy Loading
- Don't parse YAML until needed
- Don't check tmux state until necessary

### 2. Fast Path Optimization
```rust
// Quick check for common case
if name.is_none() && !Path::new(".tmuxinator.yml").exists() {
    // Fast fail without complex discovery
    return Err(TmuxrsError::ConfigNotFound(
        "No .tmuxinator.yml in current directory".to_string()
    ));
}
```

### 3. Minimal Dependencies
- Each dependency adds compile time
- Evaluate if dependency is worth it

## Release Checklist

### MVP Release (0.1.0)
- [ ] All commands work (`list`, `start`, `stop`)
- [ ] Directory detection works
- [ ] Layout support works
- [ ] Error messages are helpful
- [ ] README with examples
- [ ] Basic CI/CD setup

### Future Releases
- [ ] Git integration (0.2.0)
- [ ] Fuzzy finder support (0.3.0)
- [ ] Multi-pane windows (0.4.0)
- [ ] Hooks support (0.5.0)

*For post-MVP development strategy, see [ROADMAP.md](../ROADMAP.md).*