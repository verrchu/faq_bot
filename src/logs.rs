use std::io::{stdout, Write};

use crate::config::{Config, LogsConfig};

pub fn writer(config: &Config) -> Box<dyn Write + Send + Sync> {
    match &config.logs {
        LogsConfig::Stdout => Box::new(stdout()),
        LogsConfig::File { dir, prefix } => {
            Box::new(tracing_appender::rolling::hourly(dir, prefix))
        }
    }
}
