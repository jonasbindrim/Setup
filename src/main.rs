use clap::Parser;
use jsonschema::{Draft, JSONSchema};
use modes::{run::run_mode, validate::validate_mode, list_jobs::list_jobs_mode, list_tasks::list_tasks_mode};
use util::detect_project_file;
use std::sync::OnceLock;

use cli::Mode;

mod cli;
mod errors;
mod modes;
mod schema;
mod task_executor;
mod util;

pub static JSONSCHEMA: OnceLock<JSONSchema> = OnceLock::new();

fn main() {
    load_jsonschema();

    // Parse CLI arguments
    let args = cli::CliParameters::parse();

    let project_file = args.projectfile.unwrap_or_else(detect_project_file);

    // Execute the selected mode
    match args.mode {
        Mode::Validate => validate_mode(project_file),
        Mode::Run { job } => run_mode(project_file, job),
        Mode::ListJobs => list_jobs_mode(project_file),
        Mode::ListTasks => list_tasks_mode(project_file),
    }
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
