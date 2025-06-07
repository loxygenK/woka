use std::process::ExitCode;

use anyhow::Context as _;
use args::ConnectArgs;

use crate::{accept::common::CommonConfigSchema, config::CommonConfigs};

pub mod app;
pub mod args;
pub mod ssh;

pub fn run_connect(connect_options: ConnectArgs) -> Result<ExitCode, anyhow::Error> {
    let common: CommonConfigSchema = (&connect_options.commons).try_into()
        .context("Error during parsing arguments")?;
    let common: CommonConfigs = common.into();

    app::run_connect(&common, connect_options.server.as_deref())
        .context("Failed to connect to server")
}
