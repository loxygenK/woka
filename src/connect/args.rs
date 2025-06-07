use crate::accept::common::CommonOptionArgs;

#[derive(Debug, clap::Args)]
pub struct ConnectArgs {
    #[clap(flatten)]
    pub commons: CommonOptionArgs,

    /// The server's name to connect (optional).
    #[clap(short, long)]
    pub server: Option<String>,

    /// Port forwarding setting. Can be multiple.
    /// host:remote or host<remote to local forwarding, host>remote to remote forwarding
    #[clap(short, long)]
    pub port: Vec<String>,
}

