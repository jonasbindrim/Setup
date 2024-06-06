use setup::{
    cli::{CliParameters, Mode},
    run,
};

static JSON_BASE_PATH: &str = "./tests/project_file_parsing/settings/json/";

#[test]
/// Testing a project file with empty settings
fn empty_settings_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "empty_settings.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_ok())
}

#[test]
/// Testing a project file with settings added
fn complete_settings_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "complete_settings.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_ok())
}

#[test]
/// Testing a project file with unknown additional settings entries
fn wrong_settings_entry_test() {
    let project_file_path = format!("{}{}", JSON_BASE_PATH, "wrong_settings.json");

    let cli_args = CliParameters {
        projectfile: Some(project_file_path),
        mode: Mode::Validate,
    };

    let validation_result = run(cli_args);
    assert!(validation_result.is_err())
}
