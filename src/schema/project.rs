use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::JSONSCHEMA;

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
    pub fn import_project(project_data: Value) -> Result<Project> {
        // Validate project file against jsonschema
        Self::validate_project(&project_data)?;

        // Convert project_data to Project
        match serde_json::from_value::<Project>(project_data) {
            Ok(project) => Ok(project),
            Err(error) => Err(anyhow!(format!("Error deserializing JSON \"{}\"", error))),
        }
    }

    /// Validates a `Project` from a JSON string. Panics if the project is invalid.
    pub fn validate_project(project: &Value) -> Result<()> {
        let schema = JSONSCHEMA.get().unwrap();

        if !schema.is_valid(project) {
            Err(anyhow!(String::from(
                "Project data from does not match the json schema"
            )))
        } else {
            Ok(())
        }
    }
}
