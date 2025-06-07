use std::process::ExitCode;

use anyhow::Context as _;
use args::ConnectOptions;

use crate::{accept::common::CommonOptions, config::CommonConfigs};

pub mod app;
pub mod args;
pub mod ssh;

pub fn run_connect(connect_options: ConnectOptions) -> Result<ExitCode, anyhow::Error> {
    let common: CommonOptions = (&connect_options.commons).try_into()
        .context("Error during parsing arguments")?;
    let common: CommonConfigs = common.into();

    app::run_connect(&common, connect_options.server.as_deref())
        .context("Failed to connect to server")
}
