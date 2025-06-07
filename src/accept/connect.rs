use std::process::ExitCode;

use anyhow::Context;

use crate::apps::connect::run_connect as run_connect_app;
use crate::config::CommonConfigs;

use super::common::{CommonOptions, CommonOptionArgs};

#[derive(Debug, clap::Args)]
pub struct ConnectOptions {
    #[clap(flatten)]
    pub commons: CommonOptionArgs,

    /// The server's name to connect (optional).
    #[clap(short, long)]
    pub server: Option<String>,

    /// Port forwarding setting. Can be multiple.
    /// host:remote or host<remote to local forwarding, host>remote to remote forwarding
    #[clap(short, long)]
    pub port: Vec<String>,
}

pub fn run_connect(connect_options: ConnectOptions) -> Result<ExitCode, anyhow::Error> {
    let common: CommonOptions = (&connect_options.commons).try_into()
        .context("Error during parsing arguments")?;
    let common: CommonConfigs = common.into();

    run_connect_app(&common, connect_options.server.as_deref())
        .context("Failed to connect to server")
}

