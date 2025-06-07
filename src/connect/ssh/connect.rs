use std::{io::{BufRead, BufReader}, process::{ExitStatus, Output, Stdio}};

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
            Err(SSHConnectionError::CommandFailed(_)) => {
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

    let Ok(mut child) = cmd.spawn() else {
        return Err(SSHConnectionError::SSHExecutionFail);
    };

    let Some(stderr) = child.stderr.take() else {
        child.kill().ok();
        return Err(SSHConnectionError::SSHExecutionFail)
    };

    let mut stderr = BufReader::new(stderr);

    let mut latest_msg = String::new();
    let mut buf = String::new();
    while let Ok(size) = stderr.read_line(&mut buf) {
        if size == 0 {
            // EOF
            break;
        }
        eprint!("\x1b[38;5;248m{buf}\x1b[m");

        latest_msg = buf;
        buf = String::new();
    }

    let Ok(output) = child.wait_with_output() else {
        return Err(SSHConnectionError::SSHExecutionFail);
    };

    determine_ssh_result(&latest_msg, &output)
        .map_err(|msg| SSHConnectionError::CommandFailed(msg.to_owned()))
}

// XXX: I really need to figure out more reliable way
fn determine_ssh_result<'stderr>(stderr: &'stderr str, output: &Output) -> Result<ExitStatus, &'stderr str> {
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

