//! # tmuxrs
//!
//! A modern, Rust-based tmux session manager with centralized configuration and directory-aware execution.
//!
//! tmuxrs is a drop-in replacement for [tmuxinator](https://github.com/tmuxinator/tmuxinator) that solves
//! the scattered configuration problem while maintaining full compatibility.
//!
//! ## Key Features
//!
//! - **Drop-in tmuxinator replacement** - Use existing configs without changes
//! - **Centralized configuration** - All configs in `~/.config/tmuxrs/`
//! - **Directory-aware execution** - Auto-detects sessions from current directory
//! - **Layout support** - Complex window layouts with multiple panes
//! - **TTY-aware attachment** - Seamless terminal takeover
//!
//! ## Quick Start
//!
//! ```bash
//! # Install tmuxrs
//! cargo install tmuxrs
//!
//! # Create a configuration
//! mkdir -p ~/.config/tmuxrs
//! cat > ~/.config/tmuxrs/myproject.yml << EOF
//! name: myproject
//! root: ~/code/myproject
//! windows:
//!   - editor:
//!       layout: main-vertical
//!       panes:
//!         - vim
//!         - rails server
//! EOF
//!
//! # Start the session
//! tmuxrs start myproject
//! ```
//!
//! ## Configuration
//!
//! tmuxrs uses YAML configuration files stored in `~/.config/tmuxrs/`. The format is fully
//! compatible with tmuxinator configurations.
//!
//! ### Simple Windows
//! ```yaml
//! name: simple-project
//! root: ~/code/simple-project
//! windows:
//!   - editor: vim
//!   - server: rails server
//! ```
//!
//! ### Layout Windows
//! ```yaml
//! name: layout-project
//! root: ~/code/layout-project
//! windows:
//!   - main:
//!       layout: main-vertical
//!       panes:
//!         - vim src/main.rs
//!         - cargo watch -x run
//! ```
//!
//! ## Core Innovation
//!
//! tmuxrs combines **centralized configuration management** with **directory-aware execution**:
//!
//! - All configurations live in `~/.config/tmuxrs/`
//! - `tmuxrs start` auto-detects the right config based on your current directory
//! - No more scattered `.tmuxinator.yml` files across projects
//! - Works regardless of where your project is located

pub mod cli;
pub mod config;
pub mod error;
pub mod session;
pub mod tmux;
