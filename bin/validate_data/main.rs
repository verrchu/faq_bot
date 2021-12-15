mod cli;
use cli::Cli;

mod language;
use language::Language;

use clap::Parser;

fn main() {
    let args = Cli::parse();

    println!("ARGS: {:?}", args);
}
