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
        /// Attach to session after creation or to existing session
        #[arg(long, default_value = "true")]
        attach: bool,
        /// Do not attach to session (overrides --attach)
        #[arg(long)]
        no_attach: bool,
        /// Add windows to existing session instead of creating new one
        #[arg(long)]
        append: bool,
    },
    /// List available session configurations
    List,
    /// Stop a tmux session
    Stop {
        /// Session name to stop
        name: String,
    },
}
