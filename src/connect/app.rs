use std::process::ExitCode;

use crate::config::{CommonConfigs, SSHServer};

use super::ssh;

pub struct ConnectOptions<'common> {
    pub configs: &'common CommonConfigs,
    pub server: &'common SSHServer,
    pub port_forwards: Vec<PortForward>,
    pub cmds: Vec<String>,
    pub interactive_shell: bool,
}

#[derive(Clone, Debug)]
pub enum PortForward {
    Local(u32, u32),
    Remote(u32, u32),
}

impl PortForward {
    pub fn local_port(&self) -> u32 {
        *match self {
            PortForward::Local(local, _) => local,
            PortForward::Remote(local, _) => local,
        }
    }

    pub fn remote_port(&self) -> u32 {
        *match self {
            PortForward::Local(_, remote) => remote,
            PortForward::Remote(_, remote) => remote,
        }
    }
}

pub fn run_connect(options: ConnectOptions) -> Result<ExitCode, ConnectError> {
    let exit = ssh::connect_server(&options)?;

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
    SSHError(
        #[source]
        #[from]
        ssh::SSHConnectionError,
    ),
}
