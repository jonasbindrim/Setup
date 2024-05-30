use clap::Parser;

use setup::run;
use setup::cli::CliParameters;

pub mod cli;

fn main() {
    // Parse CLI arguments
    let args = CliParameters::parse();

    run(args);
}
