use std::collections::HashMap;

use crate::config::{CommonConfigs, Defaults, SSHServer, Server};

#[derive(Debug, serde::Deserialize)]
pub struct CommonOptions {
    #[serde(default)]
    pub default: DefaultOptions,
    pub server: HashMap<String, ServerOption>
}
impl From<CommonOptions> for CommonConfigs {
    fn from(value: CommonOptions) -> Self {
        Self {
            defaults: value.default.into(),
            server: convert_server_map(value.server),
        }
    }
}

fn convert_server_map(map: HashMap<String, ServerOption>) -> HashMap<String, Server> {
    map.into_iter().map(|(key, ServerOption::Ssh(value))| {
        let server: SSHServer  = (key.clone(), value).into();
        (key, Server::SSH(server))
    }).collect()
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct DefaultOptions {
    pub server: Option<String>,
}
impl From<DefaultOptions> for Defaults {
    fn from(value: DefaultOptions) -> Self {
        Self {
            server: value.server
                .and_then(|server| if server.is_empty() { None } else { Some(server) })
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerOption {
    Ssh(SSHServerOption)
}

#[derive(Debug, serde::Deserialize)]
pub struct SSHServerOption {
    #[serde(default)]
    pub ssh_hosts: Vec<String>,
}
impl From<(String, SSHServerOption)> for SSHServer {
    fn from((key, value): (String, SSHServerOption)) -> Self {
        let mut ssh_hosts = value.ssh_hosts;

        if ssh_hosts.is_empty() {
            ssh_hosts.push(key.clone());
        }

        Self {
            display_name: key,
            trying_hostname: ssh_hosts,
        }
    }
}

