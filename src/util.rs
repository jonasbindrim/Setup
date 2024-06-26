use anyhow::Result;
use std::{path::Path, str::FromStr};
use termion::color;

use serde_json::Value;

/// Imports the json content of a projectfile
pub fn import_project_value(projectfile: &str) -> Result<Value> {
    let project_data = std::fs::read_to_string(projectfile);
    let project_data = match project_data {
        Ok(data) => data,
        Err(error) => {
            return Err(anyhow::anyhow!(format!(
                "Cannot read file '{projectfile}' ({error})"
            )));
        }
    };

    let value = Value::from_str(&project_data);
    match value {
        Ok(value) => Ok(value),
        Err(error) => Err(anyhow::anyhow!(format!(
            "Cannot parse JSON '{projectfile}' ({error})"
        ))),
    }
}

/// Tries to auto detect the project file
pub fn detect_project_file() -> Result<String> {
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
            return Ok(path_to_check);
        } else {
            path_to_check = format!("../{}", path_to_check);
        }
    }

    Err(anyhow::anyhow!("Could not auto detect project file"))
}

/// Formats an error message with color
pub fn format_error(error: String) -> String {
    format!(
        "{}[ERROR]  :{} {}",
        color::Fg(color::Red),
        color::Fg(color::Reset),
        error
    )
}

/// Prints a formatted and colored line to the console
pub fn print_message(severity: MessageSeverity, message: String) {
    match severity {
        MessageSeverity::Info => println!(
            "{}[INFO]    :{} {}",
            color::Fg(color::Blue),
            color::Fg(color::Reset),
            message
        ),
        MessageSeverity::Success => println!(
            "{}[SUCCESS] :{} {}",
            color::Fg(color::Green),
            color::Fg(color::Reset),
            message
        ),
        MessageSeverity::ChildInfo => println!(
            "{}[CHILDI]  :{} {}",
            color::Fg(color::Yellow),
            color::Fg(color::Reset),
            message
        ),
        MessageSeverity::ChildError => println!(
            "{}[CHILDERR]:{} {}",
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
    ChildInfo,
    ChildError,
}
