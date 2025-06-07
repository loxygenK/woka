use crate::accept::common::CommonOptionArgs;
use std::{num::ParseIntError, process::ExitCode, str::FromStr};

use anyhow::Context as _;
use clap::{
    builder::TypedValueParser,
    error::{ContextKind, ContextValue, ErrorKind},
};

use crate::{accept::common::CommonConfigSchema, config::CommonConfigs};

use super::app;

#[derive(Debug, clap::Args)]
pub struct ConnectArgs {
    #[clap(flatten)]
    pub commons: CommonOptionArgs,

    /// The server's name to connect (optional).
    #[clap(short, long)]
    pub server: Option<String>,

    /// Port forwarding setting. Can be multiple.
    /// host:remote or host<remote to local forwarding, host>remote to remote forwarding
    #[clap(
        short,
        long,
        value_parser = clap::value_parser!(app::PortForward)
    )]
    pub port: Vec<app::PortForward>,
}

impl FromStr for app::PortForward {
    type Err = PortForwardError;

    /// Try to parse one of these:
    ///   - \d+                  .... Local port forwarding to the same port number
    ///        e.g.) "3000", "4000"
    ///   - \d+:\d+ or \d+<\d+   .... Local port forwarding to the specific port number
    ///        e.g.) "3000:3000", "5432<54320"
    ///   - \d+>\d+              .... Remote port forwarding to the specific port number
    ///        e.g.) "8080>80"

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.chars().all(|char| char.is_digit(10)) {
            // [0-9]^
            let port_num = u32::from_str(str)?;
            return Ok(Self::Local(port_num, port_num));
        }

        let mut segments = str.split_inclusive(&[':', '<', '>']);

        // An agonizing-to-look diagram to show what variable corresponds to where
        // (i suck at writing a parser)
        //
        // left_and_sep        remote_port
        // |__________ ________|
        // 1 2 3 4 5 : 5 4 3 2 1
        // |~~~~~~~~ |
        // local_port \_separator

        let left_and_sep = segments.next().ok_or(PortForwardError::FormatError)?;
        let remote_port = segments
            .next()
            .ok_or(PortForwardError::FormatError)?
            .parse()?;

        if segments.next().is_some() {
            // There are too many segments
            return Err(PortForwardError::FormatError);
        }

        let Some((local_port, separator)) = left_and_sep.split_at_checked(left_and_sep.len() - 1)
        else {
            // left_and_sep was not made of `local_port` and `separator`
            return Err(PortForwardError::FormatError);
        };

        let local_port = local_port.parse()?;

        if separator == "<" {
            return Ok(Self::Local(local_port, remote_port));
        }

        if separator == ":" || separator == ">" {
            return Ok(Self::Remote(local_port, remote_port));
        }

        unreachable!("All separator patterns should have been covered, but reached to no branch");
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PortForwardError {
    #[error("The port number could not be parsed: {}", .0)]
    InvalidPortNumber(#[from] ParseIntError),

    #[error(
        "Invalid port forwarding argument:\n  The format should be 'local:remote' or 'local<remote' to local forwarding, or 'local>remote' to remote forwarding, like '3000:3000'"
    )]
    FormatError,
}

pub fn run_connect(connect_options: ConnectArgs) -> Result<ExitCode, anyhow::Error> {
    let common: CommonConfigSchema = (&connect_options.commons)
        .try_into()
        .context("Error during parsing arguments")?;
    let common: CommonConfigs = common.into();

    super::app::run_connect(app::ConnectOptions {
        configs: &common,
        target: connect_options.server.as_deref(),
        port_forwards: connect_options.port,
    })
    .context("Failed to connect to server")
}
