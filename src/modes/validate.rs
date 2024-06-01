use crate::{
    schema::project::Project,
    util::{import_project_value, print_message, MessageSeverity},
};
use anyhow::Result;

/// Executes validate mode. This mode checks whether the project file follows the jsonschema correctly.
/// - `return` - Returns whether the the execution of the mode finished successfully or not.
pub fn validate_mode(projectfile: String) -> Result<()> {
    print_message(
        MessageSeverity::Info,
        format!("Validating project file \"{}\"", projectfile),
    );

    let project_data = import_project_value(&projectfile)?;
    Project::validate_project(&project_data)?;
    print_message(
        MessageSeverity::Success,
        String::from("Project file is valid"),
    );

    Ok(())
}
