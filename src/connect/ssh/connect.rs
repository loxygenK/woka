use std::process::{ExitStatus, Output, Stdio};

use crate::{config::Server, connect::app::PortForward, log};

use super::cmd::construct_ssh_cmd;

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

        let connection_result = try_connect_to_hostname(hostname, port_forwards); 
        
        match connection_result {
            Ok(status) => {
                log!("Connection to {} closed with {}.", hostname, status);
                return Ok(status);
            }
            Err(SSHConnectionError::CommandFailed(msg)) => {
                log!("     ... Failed: {}", msg);
                continue;
            }
            Err(other_error) => {
                println!("Error connecting to {}: {}", hostname, other_error);
                return Err(other_error);
            }
        }
    }
    
    Err(SSHConnectionError::AllHostsFailed {
        server_name: server.display_name.clone(),
        attempted_hosts: server.trying_hostname.clone(),
    })
}

fn try_connect_to_hostname(hostname: &str, port_forwards: &[PortForward]) -> Result<ExitStatus, SSHConnectionError> {
    let mut cmd = construct_ssh_cmd(hostname, port_forwards);

    cmd.stdin(Stdio::inherit())
       .stdout(Stdio::inherit())
       .stderr(Stdio::piped());

    // TODO: Important message (like port bound loss) is not catched
    let Ok(output) = cmd.output() else {
        return Err(SSHConnectionError::SSHExecutionFail);
    };

    determine_ssh_result(&output)
        .map_err(|msg| SSHConnectionError::CommandFailed(msg.to_owned()))
}

// XXX: I really need to figure out more reliable way
fn determine_ssh_result(output: &Output) -> Result<ExitStatus, &str> {
    let Ok(stderr) = std::str::from_utf8(output.stderr.as_slice()) else {
        return Ok(output.status);
    };

    if stderr.starts_with("ssh: ") {
        Err(&stderr.trim())
    } else {
        Ok(output.status)
    }

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

