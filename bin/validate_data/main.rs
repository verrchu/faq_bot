mod cli;
use cli::Cli;

mod language;
use language::Language;

mod validate;
use validate::validate;

use std::io::stdout;

use anyhow::{anyhow, Error as AnyError, Result as AnyResult};
use clap::Parser;

fn main() -> AnyResult<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Cli::parse();

    if !args.path.exists() {
        return Err(anyhow!("Path {:?} does not exist", args.path));
    }

    validate(&args.path);

    Ok(())
}
