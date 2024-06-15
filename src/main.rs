use std::process::ExitCode;
use std::time::SystemTime;

use clap::Parser;

use setup::cli::CliParameters;
use setup::run;
use setup::util::format_error;

pub mod cli;

fn main() -> ExitCode {
    // Parse CLI arguments
    let args = CliParameters::parse();

    // Timestamp of the systemtime when the last interrupt was received.
    let mut interrupt_timestamp: Option<SystemTime> = None;

    // Setup ctrl+c handler
    let _ = ctrlc::set_handler(move || {
        // Check if the last interrupt was received within the last 2 seconds
        if let Some(last_interrupt) = interrupt_timestamp {
            if last_interrupt.elapsed().unwrap().as_secs() < 2 {
                // If so, exit the program
                std::process::exit(1);
            }
        }

        // Update the last interrupt timestamp
        interrupt_timestamp.replace(SystemTime::now());
    });

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", format_error(format!("{}", error)));
            ExitCode::FAILURE
        }
    }
}
