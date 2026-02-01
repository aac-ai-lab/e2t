//! Binário E2T: mapeamento emoji → token/word.

use e2t::cli::{Cli, run};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    std::process::exit(run(cli));
}
