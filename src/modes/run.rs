use crate::{schema::project::Project, util::import_project_value};

/// Executes run mode
pub fn run_mode(projectfile: String, job: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // Execute job
    println!("Executing job: {}", &job);
    project.execute_job(&job);
    println!("Job {} executed successfully", &job);
}
