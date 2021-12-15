use clap::Parser;

use super::Language;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long)]
    path: String,
    #[clap(short, long, multiple_values = true, required = true)]
    languages: Vec<Language>,
}
