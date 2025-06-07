mod schema;

use std::path::PathBuf;

pub use schema::CommonConfigSchema;

#[derive(Debug, clap::Args)]
pub struct CommonOptionArgs {
    #[clap(short = 'C', long)]
    config: Option<PathBuf>,
}

#[derive(thiserror::Error, Debug)]
pub enum CommonOptionsError {
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("The config file was not found: {0}")]
    ConfigMissing(PathBuf),

    #[error("Configuration file is not valid: {0}")]
    MalformedConfigError(#[from] toml::de::Error),
}

impl TryFrom<&CommonOptionArgs> for CommonConfigSchema {
    type Error = CommonOptionsError;

    fn try_from(value: &CommonOptionArgs) -> Result<Self, Self::Error> {
        let config_file = value.config.clone().unwrap_or_else(default_config_file);

        if !config_file.exists() {
            return Err(CommonOptionsError::ConfigMissing(config_file));
        }

        let config_file = std::fs::read_to_string(config_file)?;
        let config = toml::from_str(&config_file)?;

        Ok(config)
    }
}

#[inline]
fn default_config_file() -> PathBuf {
    let home_dir = None
        .or_else(|| std::env::var_os("HOME"))
        .or_else(|| std::env::var_os("USERPROFILE"))
        .ok_or_else(|| std::env::current_dir())
        .expect("Could not retrieve $HOME / %userprofile% / current directory for configuration file path - specify the config file manually with '--config'");

    PathBuf::from(home_dir).join(".config").join("woka").join("woka.toml")
}


