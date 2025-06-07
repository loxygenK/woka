use std::collections::HashMap;

use crate::config::{CommonConfigs, Defaults, SSHServer, Server};

#[derive(Debug, serde::Deserialize)]
pub struct CommonConfigSchema {
    #[serde(default)]
    pub default: DefaultSchema,
    pub server: HashMap<String, ServerSchema>
}
impl From<CommonConfigSchema> for CommonConfigs {
    fn from(value: CommonConfigSchema) -> Self {
        Self {
            defaults: value.default.into(),
            server: convert_server_map(value.server),
        }
    }
}

fn convert_server_map(map: HashMap<String, ServerSchema>) -> HashMap<String, Server> {
    map.into_iter().map(|(key, ServerSchema::Ssh(value))| {
        let server: SSHServer  = (key.clone(), value).into();
        (key, Server::SSH(server))
    }).collect()
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct DefaultSchema {
    pub server: Option<String>,
}
impl From<DefaultSchema> for Defaults {
    fn from(value: DefaultSchema) -> Self {
        Self {
            server: value.server
                .and_then(|server| if server.is_empty() { None } else { Some(server) })
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerSchema {
    Ssh(SSHServerSchema)
}

#[derive(Debug, serde::Deserialize)]
pub struct SSHServerSchema {
    #[serde(default)]
    pub ssh_hosts: Vec<String>,
}
impl From<(String, SSHServerSchema)> for SSHServer {
    fn from((key, value): (String, SSHServerSchema)) -> Self {
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

