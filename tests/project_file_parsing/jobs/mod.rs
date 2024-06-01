use setup::{
    cli::{CliParameters, Mode},
    run,
};

static JSON_BASE_PATH: &str = "./tests/project_file_parsing/jobs/json/";

#[test]
/// Testing a project file with correct jobs
fn correct_jobs_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "correct_jobs.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_ok())
}

#[test]
/// Testing a project file with missing task in taskcall
fn missing_task_in_taskcall_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "missing_task.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_err())
}