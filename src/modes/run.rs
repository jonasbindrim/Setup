use crate::{
    schema::project::Project,
    util::{import_project_value, print_message, MessageSeverity},
};

/// Executes run mode
pub fn run_mode(projectfile: String, job: String) {
    // Import project file
    let project_data = import_project_value(&projectfile);
    let project = Project::import_project(project_data);

    // Check working dir
    let work_dir: Option<String> = if let Some(settings) = &project.settings {
        if let Some(value) = settings.project_file_as_work_dir {
            if value {
                Some(projectfile.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // Execute job
    print_message(MessageSeverity::Info, format!("Executing job \"{}\"", job));
    project.execute_job(&job, work_dir);
    print_message(
        MessageSeverity::Success,
        format!("Job \"{}\" executed successfully", &job),
    );
}
