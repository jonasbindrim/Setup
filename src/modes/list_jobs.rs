use crate::{
    schema::project::Project,
    util::{import_project_value, print_message, MessageSeverity},
};

/// Executes list jobs mode
pub fn list_jobs_mode(projectfile: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // List jobs
    print_message(
        MessageSeverity::Info,
        format!("Available jobs in project file \"{}\"", projectfile),
    );
    for job in project.jobs.keys() {
        print_message(MessageSeverity::Info, format!("  - \"{}\"", job));
    }
}
