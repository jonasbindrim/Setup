use std::str::FromStr;

use serde_json::Value;

/// Imports the json content of a projectfile
pub fn import_project_value(projectfile: &str) -> Value {
    let project_data = std::fs::read_to_string(projectfile).expect("Error reading project file");
    Value::from_str(&project_data).unwrap_or_else(|error| panic!("Error parsing JSON: {}", error))
}
