use std::{fs::File, path::PathBuf, time::Duration};

use super::bot;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "humantime_serde")]
    pub interval: Duration,

    pub bot: bot::Config,
}

impl Config {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let file = File::open(path).map_err(anyhow::Error::from)?;
        serde_yaml::from_reader(file).map_err(anyhow::Error::from)
    }
}
