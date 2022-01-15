use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DbConfig {
    pub nodes: Vec<Url>,
    pub scripts_path: PathBuf,
    #[serde(default = "default::pool_size")]
    pub pool_size: u8,
    #[serde(with = "humantime_serde", default = "default::connection_timeout")]
    pub connection_timeout: Duration,
}

mod default {
    use super::*;

    pub fn pool_size() -> u8 {
        10
    }

    pub fn connection_timeout() -> Duration {
        Duration::from_secs(3)
    }
}
