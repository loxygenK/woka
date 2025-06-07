use std::process::ExitCode;

use clap::Parser;
use connect::{run_connect, ConnectOptions};
use server::ServerOptions;

pub mod common;
pub mod connect;
pub mod server;

/// Work At - Connect to the remote development server at ease
#[derive(Debug, clap::Parser)]
// https://github.com/clap-rs/clap/issues/3857#issuecomment-1239419407
#[clap(args_conflicts_with_subcommands = true)]
pub struct WokaArgs {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[clap(flatten)]
    pub connect_options: ConnectOptions,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Connect to the server.
    Connect(ConnectOptions),

    /// Manage servers
    Server(ServerOptions),
}

pub fn parse_and_run() -> anyhow::Result<ExitCode> {
    run_parsed_cmdline(WokaArgs::parse())
}

pub fn run_parsed_cmdline(cmdline: WokaArgs) -> anyhow::Result<ExitCode> {
    match cmdline {
        WokaArgs {
            command: None,
            connect_options,
        }
        | WokaArgs {
            command: Some(Command::Connect(connect_options)),
            ..
        } => {
            run_connect(connect_options)
        }
        WokaArgs {
            command: Some(Command::Server(server_options)),
            ..
        } => {
            todo!("Server: {server_options:#?}");
        }
    }
}


