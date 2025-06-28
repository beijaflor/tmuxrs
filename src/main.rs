mod cli;
mod config;
mod error;
mod session;
mod tmux;

use clap::Parser;
use cli::{Args, Command};
use error::Result;
use session::SessionManager;

fn main() -> Result<()> {
    let args = Args::parse();
    let session_manager = SessionManager::new();

    match args.command {
        Command::Start {
            name,
            attach,
            no_attach,
            append,
        } => {
            // Determine final attach behavior: --no-attach overrides --attach
            let should_attach = if no_attach { false } else { attach };

            let result = session_manager.start_session_with_options(
                name.as_deref(),
                None,
                should_attach,
                append,
            )?;
            println!("{}", result);
        }
        Command::List => {
            let configs = session_manager.list_configs(None)?;
            if configs.is_empty() {
                println!("No configurations found");
            } else {
                println!("Available configurations:");
                for config in configs {
                    let root = config.root.as_deref().unwrap_or("~");
                    println!(
                        "  {} - {} ({} windows)",
                        config.name,
                        root,
                        config.windows.len()
                    );
                }
            }
        }
        Command::Stop { name } => {
            let result = session_manager.stop_session(&name)?;
            println!("{}", result);
        }
    }

    Ok(())
}
