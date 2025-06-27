# dmux Architecture

## Core Philosophy

dmux is a directory-agnostic tmux workspace manager that emphasizes flexibility, simplicity, and scriptability. Unlike project-specific tools, dmux applies configurable workspace templates to any directory.

## Module Organization

### Primary Modules
- **`main.rs`**: Application orchestrator handling execution flow and error management
- **`app.rs`**: CLI argument parsing and configuration management
- **`tmux.rs`**: Core tmux integration and workspace setup logic
- **`select.rs`**: Interactive directory selection with `fzf` and `fd` integration

### Command Dispatch Pattern
- **`CommandType` Enum**: Central dispatcher for different operations
  - `Open`: Open workspace in specified/selected directory
  - `Select`: Interactive directory selection mode
  - `Pull`: Git repository cloning with workspace setup
  - `Layout`: Layout management operations

## Configuration System

### Hierarchical Configuration Loading
Priority order (highest to lowest):
1. **CLI Arguments**: Direct command-line overrides
2. **Environment Variables**: `DMUX_*` prefixed variables
3. **Configuration Files**: Multiple format support
4. **Default Values**: Built-in fallbacks

### Configuration File Support
- **Locations**: 
  - `~/.dmux.conf.*`
  - `~/.config/dmux/dmux.conf.*`
  - `$XDG_CONFIG_HOME/dmux/dmux.conf.*`
- **Formats**: JSON, YAML, TOML, HJSON
- **Loading**: `config_file_settings()` handles format detection and parsing

### Core Configuration Structure (`WorkSpaceArgs`)
```rust
struct WorkSpaceArgs {
    layout: String,           // tmux layout specification
    session_name: String,     // tmux session name
    number_of_panes: u32,     // pane count
    search_dir: PathBuf,      // directory search root
    commands: Vec<String>,    // initial commands per pane
    window_name: String,      // tmux window name
}
```

## Tmux Workspace Management

### State-Aware Session Management
dmux implements intelligent session/window handling:

1. **Session Check**: `tmux.has_session(session_name)`
2. **Window Check**: `Windows::get(window_name)` within session
3. **Creation Logic**:
   - No session: Create session + window
   - Session exists, no window: Create window in existing session
   - Both exist: Attach to existing window

### Workspace Setup Process (`setup_workspace`)
1. **Path Validation**: Ensure target directory exists
2. **Session/Window Discovery**: Check existing tmux state
3. **Creation or Attachment**: Based on discovery results
4. **Pane Configuration**: Split window according to specification
5. **Layout Application**: Apply tmux layout string
6. **Command Execution**: Send initial commands to each pane
7. **Client Attachment**: Switch or attach tmux client

### Pane Management (`setup_panes_with_commands`)
- **Window Splitting**: `tmux.split_window()` for each additional pane
- **Layout Setting**: `tmux.select_layout(layout_string)`
- **Command Distribution**: `tmux.send_keys()` for initial commands
- **Index Management**: Proper pane targeting for command execution

## Data Structures

### Primary Data Types
- **`WorkSpaceArgs`**: Configuration container with serde deserialization
- **`WorkSpace`**: Runtime workspace representation with path and metadata
- **`CommandType`**: Enum-based command dispatch system

### External Tool Integration
- **`tmux_interface` Crate**: Rust wrapper for tmux CLI commands
- **Builder Pattern Usage**: Chained method calls for tmux command construction
- **Process Management**: System command execution with error handling

## Directory Selection System

### Selection Mechanisms
- **Direct Path**: Command-line argument specification
- **Interactive (`fzf`)**: Fuzzy finder integration for directory selection
- **Fast Search (`fd`)**: Optional `fd` integration for improved performance
- **Environment Detection**: Automatic tool detection and fallback

### Selection Flow
1. **Tool Detection**: Check for `fzf` and optionally `fd`
2. **Directory Enumeration**: Build selectable directory list
3. **User Interaction**: Present fuzzy finder interface
4. **Path Resolution**: Convert selection to absolute path
5. **Workspace Opening**: Pass selected path to workspace setup

## Key Design Patterns

### Command Pattern via Enums
- **`CommandType`**: Clean separation of command logic
- **Pattern Matching**: Rust's match expressions for command dispatch
- **Extensibility**: Easy addition of new command types

### Configuration as Data
- **`WorkSpaceArgs`**: Serializable configuration structure
- **Multi-Source Merging**: Hierarchical configuration combination
- **Type Safety**: Rust's type system for configuration validation

### External Tool Abstraction
- **Process Isolation**: Clean interfaces to `tmux`, `fzf`, `fd`, `git`
- **Error Propagation**: `anyhow` crate for consistent error handling
- **Dependency Management**: Graceful degradation when tools unavailable

### Builder Pattern (Implicit)
- **Tmux Command Construction**: Chained method calls via `tmux_interface`
- **Fluent Interface**: Natural command building syntax
- **Type Safety**: Compile-time validation of command parameters

## Unique Selling Points

### Directory Agnostic Design
- **Template Reusability**: Same workspace config for any directory
- **Scriptability**: Easy integration into shell scripts and automation
- **Flexibility**: Runtime configuration via CLI arguments

### Multi-Format Configuration
- **Format Freedom**: Choose preferred configuration syntax
- **Migration Support**: Easy transition between formats
- **Tool Integration**: Works with existing configuration management

### Intelligent Session Management
- **State Awareness**: Avoids duplicate session/window creation
- **Graceful Handling**: Smart attachment to existing sessions
- **Context Preservation**: Maintains tmux session state

## Key Insights for tmuxrs

1. **Directory Agnostic Approach**: Consider templates that work across projects
2. **Multi-Format Config**: Supporting multiple config formats improves adoption
3. **State-Aware Logic**: Check existing tmux state before creating sessions
4. **External Tool Integration**: Clean abstractions for tool dependencies
5. **Configuration Hierarchy**: Multiple config sources with clear precedence
6. **Builder Pattern**: Fluent interfaces for command construction
7. **Error Handling**: Consistent error propagation and user feedback
8. **Tool Detection**: Graceful handling of optional dependencies