use std::{path::Path, process::exit, str::FromStr};
use termion::color;

use serde_json::Value;

/// Imports the json content of a projectfile
pub fn import_project_value(projectfile: &str) -> Value {
    let project_data = std::fs::read_to_string(projectfile).unwrap_or_else(|error| {
        print_message(
            MessageSeverity::Error,
            format!("Error reading project file \"{}\"", error),
        );
        exit(1);
    });

    Value::from_str(&project_data).unwrap_or_else(|error| {
        print_message(
            MessageSeverity::Error,
            format!("Error parsing JSON \"{}\"", error),
        );
        exit(1);
    })
}

/// Tries to auto detect the project file
pub fn detect_project_file() -> String {
    print_message(
        MessageSeverity::Info,
        String::from("Trying to auto detect project file..."),
    );

    let mut path_to_check = String::from("Setup.json");

    for _ in 0..25 {
        let path = Path::new(&path_to_check);
        if path.exists() {
            print_message(
                MessageSeverity::Success,
                format!("Detected project file \"{}\"", path.display()),
            );
            return path_to_check;
        } else {
            path_to_check = format!("../{}", path_to_check);
        }
    }
    print_message(
        MessageSeverity::Error,
        String::from("Could not auto detect project file"),
    );
    exit(1);
}

/// Prints a formatted and colored line to the console
pub fn print_message(severity: MessageSeverity, message: String) {
    match severity {
        MessageSeverity::Info => println!(
            "{}[INFO]   :{} {}",
            color::Fg(color::Blue),
            color::Fg(color::Reset),
            message
        ),
        MessageSeverity::Success => println!(
            "{}[SUCCESS]:{} {}",
            color::Fg(color::Green),
            color::Fg(color::Reset),
            message
        ),
        MessageSeverity::Error => println!(
            "{}[ERROR]  :{} {}",
            color::Fg(color::Red),
            color::Fg(color::Reset),
            message
        ),
    }
}

/// Represents the severity of a message written to the console
pub enum MessageSeverity {
    Info,
    Success,
    Error,
}
