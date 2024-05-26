use clap::Parser;
use jsonschema::{Draft, JSONSchema};
use std::sync::OnceLock;

use schema::project::Project;

mod cli;
mod errors;
mod schema;
mod task_executor;

pub static JSONSCHEMA: OnceLock<JSONSchema> = OnceLock::new();

fn main() {
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

    // Parse CLI arguments
    let args = cli::CliParameters::parse();

    // Import project file
    let project_file = std::fs::read_to_string(&args.projectfile).expect("Error reading project file");
    let project = Project::import_project(&project_file);

    // Execute job
    println!("Executing job: {}", &args.job);
    project.execute_job(&args.job);
    println!("Job {} executed successfully", &args.job);
}
