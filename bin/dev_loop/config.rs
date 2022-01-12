use std::{fs::File, net::Ipv4Addr, path::PathBuf, time::Duration};

use super::bot;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    #[serde(with = "humantime_serde")]
    pub interval: Duration,

    pub bot: bot::Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub scripts_path: PathBuf,
}

impl Config {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let file = File::open(path).map_err(anyhow::Error::from)?;
        serde_yaml::from_reader(file).map_err(anyhow::Error::from)
    }
}
