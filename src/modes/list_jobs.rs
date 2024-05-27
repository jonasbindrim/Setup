use crate::{schema::project::Project, util::import_project_value};

/// Executes list jobs mode
pub fn list_jobs_mode(projectfile: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // List jobs
    println!("Available jobs:");
    for job in project.jobs.keys() {
        println!("{}", job);
    }
}