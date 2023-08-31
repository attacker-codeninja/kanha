use crate::{
    commands::statuscode::statc::handle_status_command,
    interface::args::{Cli, CommandChoice},
};
use clap::Parser;
use std::result::Result;

mod commands;
mod interface;
mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        CommandChoice::Status(status_args) => {
            handle_status_command(status_args).await?;
        }

        CommandChoice::Fuzzer(fuzz_args) => {
            crate::commands::fuzz::fuzer::fuzz_url(fuzz_args).await?;
        }
    }

    Ok(())
}
