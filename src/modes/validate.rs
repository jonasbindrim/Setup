use crate::{
    schema::project::Project,
    util::{import_project_value, print_message, MessageSeverity},
};

/// Executes validate mode
pub fn validate_mode(projectfile: String) {
    print_message(
        MessageSeverity::Info,
        format!("Validating project file \"{}\"", projectfile),
    );
    let project_data = import_project_value(&projectfile);

    Project::validate_project(&project_data);
    print_message(
        MessageSeverity::Success,
        String::from("Project file is valid"),
    );
}
