#[derive(Debug, clap::Args)]
pub struct ServerOptions {
    #[command(subcommand)]
    pub command: Option<ServerCommands>,
}

#[derive(Debug, clap::Subcommand)]
pub enum ServerCommands {
    /// Add new servers to connect with Woka
    Add,

    /// Show the list of configured servers
    List,
}
