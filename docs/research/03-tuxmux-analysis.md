# tuxmux Architecture

## Core Purpose

tuxmux is a Rust-based tmux session and window manager focused on modern UX patterns like fuzzy selection, Git worktree integration, and rapid session jumping.

## Module Organization

### Primary Modules
- **`cmd`**: Command implementations (`attach`, `kill`, `list`, `jump`, `wcmd`)
- **`config`**: KDL-based configuration management with layered loading
- **`mux`**: Abstraction layer for tmux interaction via `tmux-interface` crate
- **`walker`**: Git repository discovery and workspace enumeration
- **`ui`**: Fuzzy finder and interactive selection components
- **`jumplist`**: Harpoon-inspired rapid session navigation
- **`util`**: Path manipulation and configuration utilities

### Core Command: `attach`
The `attach` command is tuxmux's heart, handling:
1. Workspace discovery through `walker` module
2. Session existence checking via `mux.session_exists()`
3. Session creation or attachment logic
4. Integration with fuzzy finder for interactive selection

## Configuration System

### KDL-Based Configuration
- **Format**: Uses KDL (Cuddly Document Language) for human-friendly config files
- **Locations**: 
  - Global: `$XDG_CONFIG_HOME/tuxmux/config.kdl`
  - Local: `$XDG_DATA_HOME/tuxmux/config.kdl`
- **Precedence**: Local overrides global, with value merging for lists

### Key Configuration Fields
```kdl
search_paths {
    workspace_paths "/path/to/projects"
    single_paths "/path/to/single/project"
}
exclude_paths "node_modules" ".git" "target"
search_depth 3
default_worktree true
```

### Configuration Loading Process
1. Load default configuration
2. Parse and merge global configuration if exists
3. Parse and merge local configuration if exists
4. Final configuration used throughout application

## Data Structures

### Core Structures
- **`Config`**: Central configuration container with search paths, exclusions, depth settings
- **`SearchPath`**: Workspace and single path management for discovery
- **`Mux`**: Enum for multiplexer abstraction (currently only `Tmux` variant)
- **`Mode`**: Fuzzy finder mode enumeration

### Git Integration
- **Worktree Support**: Detects and manages multiple Git worktrees
- **Default Branch Logic**: `default_worktree` config controls automatic selection
- **Repository Discovery**: Walker module recursively finds Git repositories

## Session Management Strategy

### Session Lifecycle
1. **Discovery**: Walker finds workspaces within configured search paths
2. **Selection**: User selects via fuzzy finder or direct specification
3. **Existence Check**: `mux.session_exists()` determines if session already running
4. **Action Decision**:
   - Existing session: Attach via `mux.attach_session()`
   - New session: Create via `mux.create_session()` then attach

### Tmux Integration Abstraction
- **`tmux-interface` Crate**: Rust wrapper around tmux CLI commands
- **Command Generation**: Translates high-level operations to tmux commands
- **Session Operations**: `list-sessions`, `has-session`, `new-session`, `attach-session`, `kill-session`

## Key Design Patterns

### Strategy Pattern
- **`Mux` Enum**: Allows future multiplexer implementations beyond tmux
- **Extensible Design**: Ready for additional terminal multiplexers

### Configuration Layers
- **Hierarchical Merging**: Default → Global → Local with intelligent value combination
- **Environment Integration**: Respects XDG Base Directory Specification

### Error Handling
- **`miette` Crate**: Rich error reporting with source code context
- **Contextual Errors**: Detailed debugging information for configuration and command failures

## Unique Features

### Jump List (Harpoon-inspired)
- **Rapid Navigation**: Quick session switching without fuzzy finder
- **Session Memory**: Remembers frequently accessed sessions
- **Keyboard-Driven**: Optimized for fast, repetitive session switching

### Git Worktree Intelligence
- **Multi-Worktree Projects**: Handles repositories with multiple worktrees
- **Smart Selection**: Automatically chooses appropriate worktree or prompts user
- **Branch Awareness**: Integrates Git branch information into session management

### Workspace Discovery
- **Recursive Search**: Configurable depth for repository discovery
- **Path Exclusion**: Smart filtering of common non-project directories
- **Performance Optimized**: Efficient traversal of large directory structures

## Key Insights for tmuxrs

1. **KDL Configuration**: Consider KDL as alternative to YAML for better UX
2. **Fuzzy Selection**: Interactive selection greatly improves usability
3. **Git Integration**: Git-aware features are valuable for developers
4. **Abstraction Layers**: `mux` module pattern provides clean tmux interaction
5. **Jump Lists**: Quick session switching is a major productivity feature
6. **Configuration Layering**: Multiple config sources with smart merging
7. **Error Reporting**: Rich error messages improve developer experience