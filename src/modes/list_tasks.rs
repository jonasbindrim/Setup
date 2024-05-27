use crate::{schema::project::Project, util::import_project_value};

/// Executes list tasks mode
pub fn list_tasks_mode(projectfile: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // List tasks
    println!("Available tasks:");
    for task in project.tasks.keys() {
        println!("{}", task);
    }
}