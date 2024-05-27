use crate::{schema::project::Project, util::import_project_value};

/// Executes validate mode
pub fn validate_mode(projectfile: String) {
    println!("Validating project file: {}", projectfile);
    let project_data = import_project_value(&projectfile);

    Project::validate_project(&project_data);
    println!("Project file is valid");
}
