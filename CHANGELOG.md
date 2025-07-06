# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-01-06

### Fixed
- **Shell initialization** - Fixed direnv and shell environment inheritance in tmux sessions (#4)
  - Windows and panes now properly initialize shells before running commands
  - Shell hooks (like direnv) now work correctly in all windows and split panes
  - Environment variables are properly inherited from the parent shell

### Added
- **Shell interaction tests** - Comprehensive integration tests for shell initialization (#6)
- **Robust path expansion** - Added shellexpand library for proper tilde expansion in paths (#3)

### Changed
- **Window creation behavior** - Modified to create windows without commands first, then send commands via send-keys
- **Split pane creation** - Extended shell initialization fix to split panes

## [0.1.0] - 2024-06-28

### Added
- **Core session management** - Start, stop, and list tmux sessions
- **Configuration discovery** - Auto-detect sessions from directory basename
- **Tmuxinator compatibility** - Drop-in replacement with YAML config support
- **Layout support** - Create complex window layouts with multiple panes
  - `main-vertical` - Side-by-side layout with main pane on left
  - `main-horizontal` - Top/bottom layout with main pane on top  
  - `tiled` - Grid layout with equally sized panes
  - `even-horizontal` and `even-vertical` layouts
- **TTY-aware attachment** - Seamless terminal takeover with proper TTY inheritance
- **CLI flags** - `--attach`, `--no-attach`, `--append` for fine-grained control
- **Centralized configuration** - All configs in `~/.config/tmuxrs/`
- **Directory-aware execution** - Auto-detect sessions from current directory
- **Graceful existing session handling** - Attach to existing sessions instead of erroring
- **Comprehensive test suite** - 39 tests with full coverage
- **Pre-commit hooks** - Automated code quality checks

### Features
- **Commands**: `start`, `stop`, `list`
- **Configuration formats**: Simple windows, complex windows, layout windows
- **Error handling**: Detailed error messages with context
- **Performance**: Rust-based implementation faster than Ruby tmuxinator

### Technical
- **Dependencies**: clap, serde, serde_yaml, thiserror, dirs
- **MSRV**: Rust 1.70.0
- **Platforms**: Linux, macOS, Windows (where tmux is available)
- **License**: MIT OR Apache-2.0

[Unreleased]: https://github.com/beijaflor/tmuxrs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/beijaflor/tmuxrs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/beijaflor/tmuxrs/releases/tag/v0.1.0