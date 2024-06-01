use setup::{
    cli::{CliParameters, Mode},
    run,
};

static JSON_BASE_PATH: &str = "./tests/project_file_parsing/tasks/json/";

#[test]
/// Testing a project file with correct tasks
fn correct_tasks_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "correct_content_tasks.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_ok())
}

#[test]
/// Testing a project file with missing command entry
fn missing_command_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "missing_command_task.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    match &validation_result {
        Err(err) => println!("{}", err),
        _ => {}
    }
    assert!(validation_result.is_err())
}