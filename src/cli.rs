use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tmuxrs",
    about = "A modern tmux session manager",
    version = "0.1.0"
)]
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