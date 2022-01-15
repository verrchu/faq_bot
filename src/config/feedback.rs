use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedbackConfig {
    #[serde(with = "humantime_serde", default = "default::ack_ttl")]
    pub ack_ttl: Duration,
    #[serde(with = "humantime_serde", default = "default::timeout")]
    pub timeout: Duration,
}

mod default {
    use super::*;

    pub fn timeout() -> Duration {
        Duration::from_secs(60 * 2)
    }

    pub fn ack_ttl() -> Duration {
        Duration::from_secs(5)
    }
}
