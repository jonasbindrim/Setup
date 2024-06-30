mod jobs;
mod settings;
mod tasks;

use setup::{
    cli::{CliParameters, Mode},
    run,
};

static JSON_BASE_PATH: &str = "./tests/project_file_parsing/";

#[test]
/// Test the parsing of a minimal project file
fn minimal_project_file_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "minimal_project_file.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
        silent_children: None,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_ok())
}
