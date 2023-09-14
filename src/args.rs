use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Encode message into a Png file
    Encode(EncodeArgs),

    /// Decode message from a Png file
    Decode(DecodeArgs),

    /// Remove message from a Png file
    Remove(RemoveArgs),

    /// Print out chunks of Png file
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    pub input: PathBuf,

    pub chunk_type: String,

    pub message: String,

    pub output: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    pub input: PathBuf,

    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub input: PathBuf,

    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    pub input: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;

    Cli::command().debug_assert()
}
