use std::process::ExitCode;

use clap::Parser;

use setup::cli::CliParameters;
use setup::run;
use setup::util::format_error;

pub mod cli;

fn main() -> ExitCode {
    // Parse CLI arguments
    let args = CliParameters::parse();

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", format_error(format!("{}", error)));
            ExitCode::FAILURE
        }
    }
}
