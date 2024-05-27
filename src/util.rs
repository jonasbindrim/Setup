use std::{path::Path, str::FromStr};

use serde_json::Value;

/// Imports the json content of a projectfile
pub fn import_project_value(projectfile: &str) -> Value {
    let project_data = std::fs::read_to_string(projectfile).expect("Error reading project file");
    Value::from_str(&project_data).unwrap_or_else(|error| panic!("Error parsing JSON: {}", error))
}

/// Tries to auto detect the project file
pub fn detect_project_file() -> String {
    println!("Trying to auto detect project file...");
    
    let mut path_to_check = String::from("Setup.json");
    
    for _ in 0..25 {
        let path = Path::new(&path_to_check);
        if path.exists() {
            println!("Detected project file: {}", path.display());
            return path_to_check;
        } else {
            path_to_check = format!("../{}", path_to_check);
        }
    }
    panic!("Could not auto detect project file");
}
