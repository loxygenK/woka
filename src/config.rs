use std::collections::HashMap;

#[derive(Debug)]
pub struct CommonConfigs {
    pub defaults: Defaults,
    pub server: HashMap<String, Server>,
}

#[derive(Debug, Default)]
pub struct Defaults {
    pub server: String,
}

#[derive(Debug)]
pub enum Server {
    SSH(SSHServer)
} 

#[derive(Debug)]
pub struct SSHServer {
    pub display_name: String,
    pub trying_hostname: Vec<String>,
}

