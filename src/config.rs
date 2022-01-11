use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub feedback: FeedbackConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackConfig {
    #[serde(with = "humantime_serde")]
    pub ack_ttl: Duration,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}
