# tmuxrs Development Roadmap

## MVP Phase 1: Core Functionality

### Commands (Limited Scope)

1. **`tmuxrs list`**
   - List all session configurations from `~/.config/tmuxrs/*.yml`
   - Simple output: just session names

2. **`tmuxrs start <name>`**
   - Start new tmux session from `~/.config/tmuxrs/<name>.yml`
   - Attach to existing session if already running
   - Basic validation only

3. **`tmuxrs stop <name>`**
   - Kill the tmux session with the given name
   - No confirmation prompt in MVP

### Configuration Support (Minimal)

Only support these YAML fields in MVP:
```yaml
name: myproject
root: ~/projects/myproject

windows:
  - editor: vim           # Simple format: window_name: command
  - server: cargo run     # Single pane per window only
  - logs: tail -f log/dev.log
```

### Out of Scope for MVP

The following features will NOT be implemented in Phase 1:
- Directory-based auto-detection (`tmuxrs start` with no args)
- `tmuxrs current` command
- Multiple panes per window
- Pre/post hooks (`pre_window`, `on_project_start`, etc.)
- Custom tmux options
- Pane synchronization
- Complex pane configurations

### In Scope for MVP
- Basic window layouts (main-vertical, main-horizontal, etc.)

### Implementation Priorities

1. **Core tmux interaction** - Basic session create/attach/kill
2. **YAML parsing** - Simple configuration loading
3. **Error handling** - Clear messages for common failures

## Future Phases (Post-MVP)

### Phase 2: Enhanced Features
- Directory-based auto-detection
- `tmuxrs current` command
- Multiple panes per window with layouts
- Basic pre-window commands

### Phase 3: Full Tmuxinator Compatibility
- All tmuxinator YAML fields
- Project lifecycle hooks
- Pane synchronization
- Complex nested configurations

## Stretch Goals

Optional features that would differentiate tmuxrs from tmuxinator:

### Enhanced Session Management
- **Fuzzy selector**: Interactive session selection using `fzf` or built-in selector
- **Git integration**: Use Git repository name instead of directory basename for session detection
- **Git-aware root matching**: Auto-detect sessions based on git repository roots
- **Git worktree support**: Handle multiple worktrees within same repository
- **Project detection**: Shell hooks to auto-start sessions when entering directories
- **Automatic cleanup**: Remove dead sessions and stale layouts on startup

### Configuration Extensions
- **Session templates**: Base templates that other sessions can inherit from
- **Environment injection**: Load and export environment variables per session
- **Inline commands**: Support `.tmuxrs.yml` in project roots for ad-hoc sessions

### Developer Experience
- **Layout visualization**: Preview tmux layouts as ASCII diagrams before launching
- **Command aliases**: Define custom shortcuts for frequent tmux operations
- **Plugin support**: Extensibility through external commands or Rust modules
- **Advanced hooks**: Pre/post hooks at session, window, and pane levels

## Post-MVP Development Strategy

1. **Gather Feedback**: Use it yourself, share with others
2. **Profile Performance**: Where are the bottlenecks?
3. **Improve UX**: Better error messages, helpful suggestions
4. **Add Features**: Based on real usage patterns
5. **Build Community**: Good docs, responsive to issues

## Release Schedule
- **MVP (0.1.0)**: Core functionality
- **Enhanced (0.2.0)**: Directory auto-detection, Git integration
- **Advanced (0.3.0)**: Fuzzy finder, multi-pane windows
- **Complete (0.4.0)**: Full tmuxinator compatibility, hooks