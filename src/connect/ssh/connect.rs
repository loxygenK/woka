use std::process::ExitStatus;

use crate::{
    config::Server,
    connect::{
        app::PortForward,
        ssh::cmd::{SSHCommand, SSHCommandError},
    },
    log,
};

pub fn connect_server(
    Server::SSH(server): &Server,
    port_forwards: &[PortForward],
) -> Result<ExitStatus, SSHConnectionError> {
    if server.trying_hostname.is_empty() {
        return Err(SSHConnectionError::NoHostsConfigured);
    }

    log!("Connecting to server '{}'...", server.display_name);

    for hostname in &server.trying_hostname {
        log!("  -> {hostname}");

        let mut command = SSHCommand::new(hostname, port_forwards);
        match command.connect() {
            Ok(status) => {
                log!("Connection to {} closed with {}.", hostname, status);
                return Ok(status);
            }
            Err(SSHCommandError::ConnectionFailed) => {
                continue;
            }
            Err(SSHCommandError::ExecutionFailed) => {
                return Err(SSHConnectionError::SSHExecutionFail);
            }
        }
    }

    Err(SSHConnectionError::AllHostsFailed {
        server_name: server.display_name.clone(),
        attempted_hosts: server.trying_hostname.clone(),
    })
}

#[derive(thiserror::Error, Debug)]
pub enum SSHConnectionError {
    #[error("No hosts conigured for this server")]
    NoHostsConfigured,

    #[error(
        "No host could not be connected for '{}'. Tried: {}",
        .server_name,
        .attempted_hosts.join(", ")
    )]
    AllHostsFailed {
        server_name: String,
        attempted_hosts: Vec<String>,
    },

    #[error("SSH command could not be executed")]
    SSHExecutionFail,

    #[error("SSH command failed: {}", .0)]
    CommandFailed(String),
}
