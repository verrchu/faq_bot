use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum LogsConfig {
    Stdout,
    File { dir: PathBuf, prefix: String },
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self::Stdout
    }
}
