// Integration tests organized by functionality
//
// This file serves as the main entry point for all integration tests,
// organized into logical modules for better maintainability.
//
// Test Structure:
// - cli/     - CLI interface and core command tests
// - session/ - Session lifecycle management tests
// - window/  - Window and layout management tests
// - shell/   - Shell integration and environment tests
// - tmux/    - Low-level tmux operations tests
// - common/  - Shared utilities and test infrastructure

mod common;

// Test modules organized by functionality
mod cli;
mod session;
mod shell;
mod tmux;
mod window;

// The skip_if_not_integration_env macro is defined in common/mod.rs
// and is available to all test modules through the #[macro_export] attribute
