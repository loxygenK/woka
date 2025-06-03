use anyhow::Context;

use crate::config::CommonConfigs;

use super::common::{CommonOptions, CommonOptionArgs};

#[derive(Debug, clap::Args)]
pub struct ConnectOptions {
    #[clap(flatten)]
    pub commons: CommonOptionArgs,

    /// The server's name to connect (optional).
    #[clap(short, long)]
    pub server: Option<String>,
}

pub fn run_connect(connect_options: ConnectOptions) -> Result<(), anyhow::Error> {
    let common: CommonOptions = (&connect_options.commons).try_into()
        .context("Error during parsing arguments")?;
    let common: CommonConfigs = common.into();

    todo!("{common:#?}");
}

