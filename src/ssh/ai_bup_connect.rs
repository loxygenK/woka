use std::process::{Command, Stdio};

use crate::config::SSHServer;

pub struct SSHConnector {
    server: SSHServer,
}

impl SSHConnector {
    pub fn new(server: SSHServer) -> Self {
        Self { server }
    }

    pub fn connect(&self) -> Result<(), SSHConnectionError> {
        if self.server.trying_hostname.is_empty() {
            return Err(SSHConnectionError::NoHostsConfigured);
        }

        println!("Connecting to server '{}'...", self.server.display_name);
        
        for (index, hostname) in self.server.trying_hostname.iter().enumerate() {
            if index > 0 {
                println!();
            }
            println!("Trying {}...", hostname);
            
            match self.try_connect_to_hostname(hostname) {
                Ok(_) => {
                    return Ok(());
                }
                Err(SSHConnectionError::HostUnreachable) => {
                    if index < self.server.trying_hostname.len() - 1 {
                        println!("Connection failed, trying next hostname...");
                    } else {
                        println!("Connection failed.");
                    }
                    continue;
                }
                Err(SSHConnectionError::AuthenticationFailed) => {
                    println!("Authentication failed for {}", hostname);
                    continue;
                }
                Err(other_error) => {
                    println!("Error connecting to {}: {}", hostname, other_error);
                    return Err(other_error);
                }
            }
        }
        
        Err(SSHConnectionError::AllHostsFailed {
            server_name: self.server.display_name.clone(),
            attempted_hosts: self.server.trying_hostname.clone(),
        })
    }

    fn try_connect_to_hostname(&self, hostname: &str) -> Result<(), SSHConnectionError> {
        let mut cmd = Command::new("ssh");
        cmd.arg("-o").arg("ConnectTimeout=10")
           .arg("-o").arg("BatchMode=no")
           .arg(hostname);

        cmd.stdin(Stdio::inherit())
           .stdout(Stdio::inherit())
           .stderr(Stdio::inherit());

        let status = cmd.status()
            .map_err(|e| SSHConnectionError::CommandFailed(format!("Failed to execute ssh command: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            match status.code() {
                Some(255) => Err(SSHConnectionError::HostUnreachable),
                Some(1) => Err(SSHConnectionError::AuthenticationFailed),
                Some(code) => Err(SSHConnectionError::CommandFailed(format!("SSH exited with code: {}", code))),
                None => Err(SSHConnectionError::CommandFailed("SSH process was terminated by signal".to_string())),
            }
        }
    }
}

#[derive(Debug)]
pub enum SSHConnectionError {
    NoHostsConfigured,
    AllHostsFailed {
        server_name: String,
        attempted_hosts: Vec<String>,
    },
    HostUnreachable,
    AuthenticationFailed,
    CommandFailed(String),
}

impl std::fmt::Display for SSHConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSHConnectionError::NoHostsConfigured => {
                write!(f, "No hosts configured for this server")
            }
            SSHConnectionError::AllHostsFailed { server_name, attempted_hosts } => {
                write!(f, "Failed to connect to server '{}'. Attempted hosts: {}", 
                       server_name, attempted_hosts.join(", "))
            }
            SSHConnectionError::HostUnreachable => {
                write!(f, "Host is unreachable")
            }
            SSHConnectionError::AuthenticationFailed => {
                write!(f, "Authentication failed")
            }
            SSHConnectionError::CommandFailed(msg) => {
                write!(f, "SSH command failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for SSHConnectionError {}