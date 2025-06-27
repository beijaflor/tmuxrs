# tmuxrs Architecture Design

*Implementation of decisions documented in [Feature Decisions](01-feature-decisions.md). See [Implementation Guide](../guides/01-implementation-guide.md) for practical coding steps.*

## Overview

tmuxrs is an ultra-thin tmux session manager that provides centralized configuration with directory-aware execution. It maintains strict tmuxinator compatibility while solving the configuration management problem.

## Design Principles

1. **Minimal Complexity**: Direct tmux command execution, no script generation
2. **Tmuxinator Compatibility**: Drop-in replacement for existing workflows
3. **Directory Intelligence**: Smart session detection without Git dependencies
4. **Fail Fast**: Clear errors, predictable behavior
5. **Extensible Foundation**: Architecture supports future features without rewrite

## Module Structure

```
tmuxrs/
├── src/
│   ├── main.rs           # CLI entry point and command dispatch
│   ├── cli.rs            # Command-line interface using clap
│   ├── config/
│   │   ├── mod.rs        # Configuration loading and discovery
│   │   ├── parser.rs     # YAML parsing with serde_yaml
│   │   └── validator.rs  # Configuration validation
│   ├── session/
│   │   ├── mod.rs        # Session management logic
│   │   ├── builder.rs    # Session creation from config
│   │   └── manager.rs    # Session lifecycle (create/attach/stop)
│   ├── tmux/
│   │   ├── mod.rs        # Tmux command abstraction
│   │   ├── commands.rs   # Tmux command builders
│   │   └── executor.rs   # Command execution with error handling
│   └── error.rs          # Error types and handling
```

### Module Responsibilities

#### `main.rs` & `cli.rs`
- Parse command-line arguments using clap
- Dispatch to appropriate command handlers
- Handle top-level errors and exit codes

#### `config/`
- **Discovery Logic**: Find config files based on directory or name
- **YAML Parsing**: Deserialize tmuxinator-compatible YAML
- **Validation**: Ensure required fields and valid values

#### `session/`
- **Builder**: Transform config into tmux commands
- **Manager**: Handle session lifecycle (check existence, create, attach)
- **State Management**: Track session state for idempotent operations

#### `tmux/`
- **Command Abstraction**: Type-safe tmux command construction
- **Executor**: Run commands with proper error handling
- **Output Parsing**: Parse tmux query results

## Data Models

### Core Structures

```rust
// Configuration structures (matches tmuxinator format)
#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    root: Option<PathBuf>,
    windows: Vec<Window>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Window {
    Simple(String, String),  // name: command
    WithLayout {
        #[serde(flatten)]
        window: HashMap<String, WindowConfig>,
    },
}

#[derive(Deserialize, Debug)]
struct WindowConfig {
    layout: Option<String>,
    command: Option<String>,
    panes: Option<Vec<String>>,  // For future multi-pane support
}

// Runtime structures
struct Session {
    name: String,
    config: Config,
    exists: bool,
}

// Tmux command abstraction
enum TmuxCommand {
    HasSession { name: String },
    NewSession { name: String, root: Option<PathBuf>, window_name: String },
    NewWindow { target: String, name: String, command: Option<String> },
    SelectLayout { target: String, layout: String },
    AttachSession { name: String },
    KillSession { name: String },
    ListSessions,
}
```

## Command Flow

### `tmuxrs start` (no arguments)

```
1. Get current directory
2. Extract directory basename
3. Look for configs in order:
   a. ./.tmuxinator.yml (local)
   b. ~/.config/tmuxrs/{basename}.yml (centralized)
4. If found, proceed with session creation
5. If not found, error with helpful message
```

### `tmuxrs start <name>`

```
1. Look for config file:
   a. ~/.config/tmuxrs/{name}.yml
2. Load and validate configuration
3. Check if session exists (tmux has-session)
4. If exists: attach to session
5. If not exists:
   a. Create session with first window
   b. Create additional windows
   c. Apply layouts if specified
   d. Attach to session
```

### Session Creation Algorithm

```rust
fn create_session(config: &Config) -> Result<()> {
    // 1. Create session with first window
    let first_window = &config.windows[0];
    tmux_new_session(&config.name, &config.root, first_window)?;
    
    // 2. Create remaining windows
    for window in &config.windows[1..] {
        tmux_new_window(&config.name, window)?;
        
        // 3. Apply layout if specified
        if let Some(layout) = window.layout() {
            tmux_select_layout(&config.name, &window.name(), layout)?;
        }
    }
    
    // 4. Focus first window
    tmux_select_window(&config.name, 0)?;
    
    Ok(())
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
enum TmuxrsError {
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(String),
    
    #[error("Invalid YAML in {file}: {error}")]
    YamlError { file: String, error: serde_yaml::Error },
    
    #[error("tmux command failed: {0}")]
    TmuxError(String),
    
    #[error("Session '{0}' already exists")]
    SessionExists(String),
    
    #[error("No tmuxinator config found for directory '{0}'")]
    NoConfigForDirectory(String),
}
```

### Error Strategy
- **Fail Fast**: Validate early, before any tmux commands
- **Clear Messages**: User-friendly errors with actionable information
- **No Partial State**: Atomic operations where possible

## Configuration Discovery

### Search Order
1. **Local Config**: `./.tmuxinator.yml` in current directory
2. **Centralized Config**: `~/.config/tmuxrs/{identifier}.yml` 
3. **Manual Override**: Explicitly named config file

*Note: MVP supports only `.yml` extension. `.yaml` support can be added in future phases.*

### Directory Detection (MVP)
```rust
fn detect_session_name() -> Option<String> {
    std::env::current_dir()
        .ok()
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .map(String::from)
}
```

### Future Git Integration (Stretch Goal)
```rust
// Can be swapped in later without changing architecture
fn detect_session_name_git() -> Option<String> {
    // Use git2 crate to find repository name
    // Fallback to directory basename
}
```

## Testing Strategy

### Unit Tests
- **Config Parsing**: Valid/invalid YAML scenarios
- **Discovery Logic**: Directory detection edge cases
- **Command Building**: Correct tmux command generation

### Integration Tests
```rust
#[test]
fn test_session_lifecycle() {
    // 1. Create unique test session
    // 2. Verify session exists
    // 3. Attach should succeed without creating new
    // 4. Stop should remove session
}
```

### Manual Testing Checklist
- [ ] Fresh session creation
- [ ] Attach to existing session
- [ ] Layout application
- [ ] Error cases (missing config, invalid YAML)
- [ ] Tmuxinator compatibility

## Dependencies

### Essential Crates
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
thiserror = "1"
dirs = "5"  # For config directory resolution
```

### Command Execution Strategy
- Use `std::process::Command` for tmux invocation
- No tmux client library needed (keeps it thin)
- Simple string command construction

## Extension Points

### Architecture Supports Future Features
1. **Git Integration**: Swap `detect_session_name()` implementation
2. **Multi-pane**: Extend `WindowConfig` and command generation
3. **Fuzzy Selection**: Add to session discovery flow
4. **Hooks**: Extend config model and command generation
5. **Templates**: Add template resolution before session creation

### Clean Boundaries
- Config module isolated from session logic
- Tmux commands abstracted from business logic
- Discovery strategy easily replaceable

## Performance Considerations

### Fast Startup
- Minimal dependencies
- No script generation overhead
- Direct tmux command execution

### Efficient Discovery
- Basename extraction is instant
- File existence checks are fast
- No recursive directory scanning

## Security Considerations

### Command Injection Prevention
- No shell script generation
- Commands built programmatically
- User input validated and escaped

### File System Safety
- Resolve symlinks for config files
- Validate config file permissions
- No arbitrary file execution

## Migration Path from tmuxinator

### Full Compatibility
- Same YAML structure
- Same config locations supported
- Same command semantics

### Enhanced Workflow
```bash
# Old tmuxinator way
cd ~/projects/webapp
tmuxinator start webapp

# New tmuxrs way (also works!)
cd ~/projects/webapp
tmuxrs start  # Auto-detects webapp.yml
```

## Summary

tmuxrs architecture achieves:
- **Ultra-thin implementation** through direct tmux commands
- **Tmuxinator compatibility** via same YAML format
- **Smart discovery** without heavy dependencies
- **Extensible design** for future enhancements
- **Fast, predictable** session management