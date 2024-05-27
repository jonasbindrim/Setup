use crate::{schema::project::Project, util::{import_project_value, print_message, MessageSeverity}};

/// Executes run mode
pub fn run_mode(projectfile: String, job: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // Execute job
    print_message(MessageSeverity::Info, format!("Executing job \"{}\"", job));
    project.execute_job(&job);
    print_message(MessageSeverity::Success, format!("Job \"{}\" executed successfully", &job));
}
