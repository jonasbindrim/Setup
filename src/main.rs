use clap::Parser;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;
use std::{str::FromStr, sync::OnceLock};

use cli::Mode::{Validate, Run};
use schema::project::Project;

mod cli;
mod errors;
mod schema;
mod task_executor;

pub static JSONSCHEMA: OnceLock<JSONSchema> = OnceLock::new();

fn main() {
    load_jsonschema();

    // Parse CLI arguments
    let args = cli::CliParameters::parse();

    match args.mode {
        Validate { projectfile } => validate_mode(projectfile),
        Run { projectfile, job } => run_mode(projectfile, job),
    }
}

/// Executes validate mode
fn validate_mode(projectfile: String) {
    println!("Validating project file: {}", projectfile);
    let project_data = std::fs::read_to_string(&projectfile).expect("Error reading project file");
    let project_value = Value::from_str(&project_data)
        .unwrap_or_else(|error| panic!("Error parsing JSON: {}", error));

    Project::validate_project(project_value);
    println!("Project file is valid");
}

fn run_mode(projectfile: String, job: String) {
    // Import project file
    let project_data = std::fs::read_to_string(&projectfile).expect("Error reading project file");
    let project = Project::import_project(&project_data);

    // Execute job
    println!("Executing job: {}", &job);
    project.execute_job(&job);
    println!("Job {} executed successfully", &job);
}

/// Loads the JSON schema from the jsonschema.json file
fn load_jsonschema() {
    // Setup JSONSCHEMA
    let schema = include_str!("../jsonschema.json");
    let schema = serde_json::from_str(schema).expect("Error parsing JSON schema");
    let schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema);

    match schema {
        Ok(final_schema) => JSONSCHEMA.set(final_schema).unwrap(),
        Err(validation_error) => panic!("Error compiling JSON schema: {}", validation_error),
    }
}