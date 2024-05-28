use std::{collections::HashMap, process::exit};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{util::print_message, util::MessageSeverity, JSONSCHEMA};

use super::{job::Job, settings::Settings, task::Task};

/// Represents the content of a `Project` from a configuration file
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub settings: Option<Settings>,
    pub jobs: HashMap<String, Job>,
    pub tasks: HashMap<String, Task>,
}

impl Project {
    /// Imports a `Project` from a JSON string
    pub fn import_project(project_data: Value) -> Project {
        // Validate project file against jsonschema
        Self::validate_project(&project_data);

        // Convert project_data to Project
        serde_json::from_value::<Project>(project_data).unwrap_or_else(|error| {
            print_message(
                MessageSeverity::Error,
                format!("Error deserializing JSON \"{}\"", error),
            );
            exit(1);
        })
    }

    /// Validates a `Project` from a JSON string. Panics if the project is invalid.
    pub fn validate_project(project: &Value) {
        let schema = JSONSCHEMA.get().unwrap();

        if !schema.is_valid(project) {
            print_message(
                MessageSeverity::Error,
                String::from("Project data from does not match the json schema"),
            );
            exit(1);
        }
    }
}
