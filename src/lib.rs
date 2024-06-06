use anyhow::Result;
use jsonschema::{Draft, JSONSchema};
use modes::{
    list_jobs::list_jobs_mode, list_tasks::list_tasks_mode, run_job::run_job_mode,
    run_task::run_task_mode, validate::validate_mode,
};
use std::sync::OnceLock;
use util::detect_project_file;

use cli::{CliParameters, Mode};

pub mod cli;
mod modes;
mod schema;
mod task_executor;
pub mod util;

pub static JSONSCHEMA: OnceLock<JSONSchema> = OnceLock::new();

pub fn run(args: CliParameters) -> Result<()> {
    load_jsonschema();

    let project_file_path = match args.projectfile {
        Some(path) => path,
        None => detect_project_file()?,
    };

    // Execute the selected mode
    match args.mode {
        Mode::Validate => validate_mode(project_file_path),
        Mode::ListTasks => list_tasks_mode(project_file_path),
        Mode::ListJobs => list_jobs_mode(project_file_path),
        Mode::Run { job } => run_job_mode(project_file_path, job),
        Mode::RunTask { task, arguments } => run_task_mode(project_file_path, task, arguments),
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

    let _ = match schema {
        Ok(final_schema) => JSONSCHEMA.set(final_schema),
        Err(validation_error) => panic!("Error compiling JSON schema: {}", validation_error),
    };
}
