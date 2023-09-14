mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod types;

use crate::args::Command;
use clap::Parser;

fn main() {
    let cli = args::Cli::parse();
    let _ = match cli.command {
        Command::Encode(command) => commands::encode(command),
        Command::Decode(command) => commands::decode(command),
        Command::Remove(command) => commands::remove(command),
        Command::Print(command) => commands::print(command),
    };
}
