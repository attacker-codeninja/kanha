use crate::interface::splashes::show_splashes;
use clap::{Args, Parser, Subcommand};

/// The KANHA CLI.
#[derive(Parser)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// The command to execute.
    #[clap(subcommand)]
    pub command: CommandChoice,
}

#[derive(Subcommand)]
#[command(arg_required_else_help = true)]
#[command(author, version, about = show_splashes(), long_about = show_splashes())]
pub enum CommandChoice {
    /// Returns the HTTP response code of URLs
    #[command(arg_required_else_help = true)]
    #[clap(name = "status")]
    Status(StatusArgs),

    /// Fuzz URLs with the response code
    #[command(arg_required_else_help = true)]
    #[clap(name = "fuzz")]
    Fuzzer(FuzzerArgs),
}

#[derive(Args)]
pub struct StatusArgs {
    /// A url or a file containing multiple urls
    #[arg(required = false, short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,

    /// A list of the ports or separated by 2 dots
    #[arg(short, long)]
    pub ports: Option<String>,

    /// Define the maximum concurrent tasks
    #[arg(short, long, default_value = "10")]
    pub tasks: usize,
}

#[derive(Args)]
pub struct FuzzerArgs {
    /// A file containing a list of possible wordlists
    #[arg(required = true, short, long)]
    pub filename: String,

    /// Provide a url to fuzz
    #[arg(required = true, short, long)]
    pub url: String,
}
