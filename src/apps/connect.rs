use std::process::ExitCode;

use crate::{config::CommonConfigs, ssh::connect_server};

pub struct ConnectOptions<'common> {
    pub common: &'common CommonConfigs,
    pub port_forwards: Vec<PortForward>,
}

#[derive(Debug)]
pub enum PortForward {
    Local(u32, u32),
    Remote(u32, u32),
}

pub fn run_connect(configs: &CommonConfigs, server_name: Option<&str>) -> Result<ExitCode, ConnectError> {
    let target_server_name = server_name
        .or(configs.defaults.server.as_deref())
        .ok_or(ConnectError::NoServerSpecified)?;

    let server = configs.server.get(target_server_name)
        .ok_or_else(|| ConnectError::ServerNotFound(target_server_name.to_string()))?;

    let exit = connect_server(server)?;

    if let Some(code) = exit.code().and_then(|code| u8::try_from(code).ok()) {
        return Ok(code.into());
    }

    if exit.success() {
        return Ok(ExitCode::SUCCESS);
    } else {
        return Ok(ExitCode::FAILURE);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error("No server specified and no default server configured")]
    NoServerSpecified,

    #[error("Server '{}' not found in configuration", .0)]
    ServerNotFound(String),

    #[error("SSH Connection failed:\n{}", .0)]
    SSHError(#[source] #[from] crate::ssh::SSHConnectionError),
}
