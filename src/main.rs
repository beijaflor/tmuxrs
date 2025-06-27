mod cli;
mod config;
mod error;
mod session;
mod tmux;

use error::Result;

fn main() -> Result<()> {
    println!("tmuxrs - A modern tmux session manager");
    println!("Phase 0: Project structure initialized");
    Ok(())
}
