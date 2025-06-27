mod cli;
mod config;
mod error;
mod session;
mod tmux;

use clap::Parser;
use cli::{Args, Command};
use error::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Command::Start { name } => {
            println!("Starting session: {:?}", name);
            // TODO: Implement start command
        }
        Command::List => {
            println!("Listing sessions");
            // TODO: Implement list command
        }
        Command::Stop { name } => {
            println!("Stopping session: {}", name);
            // TODO: Implement stop command
        }
    }
    
    Ok(())
}
