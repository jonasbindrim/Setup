use anyhow::Result;

use crate::{
    schema::project::Project,
    util::{import_project_value, print_message, MessageSeverity},
};

/// Executes list tasks mode
pub fn list_tasks_mode(projectfile: String) -> Result<()> {
    // Import project file
    let project_data = import_project_value(&projectfile)?;
    let project = Project::import_project(project_data)?;

    // List tasks
    print_message(
        MessageSeverity::Info,
        format!("Available tasks in project file \"{}\"", projectfile),
    );
    for task in project.tasks.keys() {
        print_message(MessageSeverity::Info, format!("  - \"{}\"", task));
    }

    Ok(())
}
