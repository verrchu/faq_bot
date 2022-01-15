mod db;
pub use db::DbConfig;

mod feedback;
pub use feedback::FeedbackConfig;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub feedback: FeedbackConfig,
    pub db: DbConfig,
}
