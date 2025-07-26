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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_command_with_name() {
        let args = Args::parse_from(["tmuxrs", "start", "my-session"]);
        match args.command {
            Command::Start {
                name,
                attach,
                no_attach,
                append,
            } => {
                assert_eq!(name, Some("my-session".to_string()));
                assert!(attach);
                assert!(!no_attach);
                assert!(!append);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_parse_start_command_without_name() {
        let args = Args::parse_from(["tmuxrs", "start"]);
        match args.command {
            Command::Start {
                name,
                attach,
                no_attach,
                append,
            } => {
                assert_eq!(name, None);
                assert!(attach);
                assert!(!no_attach);
                assert!(!append);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_parse_start_command_with_no_attach() {
        let args = Args::parse_from(["tmuxrs", "start", "--no-attach"]);
        match args.command {
            Command::Start {
                name,
                attach,
                no_attach,
                append,
            } => {
                assert_eq!(name, None);
                assert!(attach); // Default value is still true
                assert!(no_attach); // But no_attach flag is set
                assert!(!append);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_parse_start_command_with_append() {
        let args = Args::parse_from(["tmuxrs", "start", "my-session", "--append"]);
        match args.command {
            Command::Start {
                name,
                attach,
                no_attach,
                append,
            } => {
                assert_eq!(name, Some("my-session".to_string()));
                assert!(attach);
                assert!(!no_attach);
                assert!(append);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_parse_list_command() {
        let args = Args::parse_from(["tmuxrs", "list"]);
        match args.command {
            Command::List => {
                // List command has no parameters
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_parse_stop_command() {
        let args = Args::parse_from(["tmuxrs", "stop", "my-session"]);
        match args.command {
            Command::Stop { name } => {
                assert_eq!(name, "my-session");
            }
            _ => panic!("Expected Stop command"),
        }
    }

    #[test]
    fn test_parse_start_with_all_flags() {
        let args = Args::parse_from([
            "tmuxrs",
            "start",
            "test-session",
            "--attach",
            "--no-attach",
            "--append",
        ]);
        match args.command {
            Command::Start {
                name,
                attach,
                no_attach,
                append,
            } => {
                assert_eq!(name, Some("test-session".to_string()));
                assert!(attach);
                assert!(no_attach);
                assert!(append);
            }
            _ => panic!("Expected Start command"),
        }
    }
}
