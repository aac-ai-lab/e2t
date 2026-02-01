//! Binário E2T: mapeamento emoji → token/word.

use clap::Parser;
use e2t::cli::{run, Cli};

fn main() {
    let cli = Cli::parse();
    std::process::exit(run(cli));
}
