use std::path::PathBuf;

use clap::Parser;

use super::Language;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long)]
    pub path: PathBuf,
    #[clap(short, long, multiple_values = true, required = true)]
    pub languages: Vec<Language>,
}
