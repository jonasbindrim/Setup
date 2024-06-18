use std::process::ExitStatus;

use anyhow::{anyhow, Result};

use crate::{
    schema::{project::Project, task_call::TaskCall},
    task_executor::TaskExecutor,
    util::{format_error, import_project_value, print_message, MessageSeverity},
};

/// Executes run task mode
pub fn run_task_mode(projectfile: String, task: String, arguments: Vec<String>) -> Result<()> {
    // Import project file
    let project_data = import_project_value(&projectfile)?;
    let project = Project::import_project(project_data)?;

    // Check working dir
    let work_dir: Option<String> = if let Some(settings) = &project.settings {
        match settings.project_file_as_work_dir {
            Some(true) => Some(projectfile.clone()),
            _ => None,
        }
    } else {
        None
    };

    // Execute task
    print_message(
        MessageSeverity::Info,
        format!("Executing task with name \"{}\"", task),
    );

    let mut task_executor = match build_executor(&project, &task, arguments, work_dir) {
        Ok(executor) => executor,
        Err(error) => {
            eprintln!("{}", format_error(format!("{}", error)));
            return Err(anyhow!(format!("Task \"{}\" failed", &task)));
        }
    };
    
    match execute_task(&mut task_executor) {
        Ok(()) => {
            print_message(
                MessageSeverity::Success,
                format!("Task \"{}\" executed successfully", &task_executor.execution_string),
            );
            Ok(())
        }
        Err(error) => {
            eprintln!("{}", format_error(format!("{}", error)));
            Err(anyhow!(format!("Task \"{}\" failed", &task_executor.execution_string)))
        }
    }
}

/// Executes the task with the given name
fn execute_task(
    task_executor: &mut TaskExecutor,
) -> Result<()> {

    task_executor.execute()?;

    let status = task_executor.wait()?;
    if !ExitStatus::success(&status) {
        return Err(anyhow!(format!(
            "Task \"{}\" failed with exit code {}",
            task_executor.execution_string,
            if let Some(status) = status.code() {
                status.to_string()
            } else {
                "unknown".to_string()
            }
        )));
    }

    Ok(())
}

/// Builds the task executor
fn build_executor(project: &Project, taskname: &str, arguments: Vec<String>, work_dir: Option<String>) -> Result<TaskExecutor> {
    // Get the task
    let Some(task) = project.tasks.get(taskname) else {
        return Err(anyhow!(format!(
            "Task with name \"{}\" not found",
            taskname
        )));
    };

    // Check matching amount of call args
    if let Some(needed_args) = task.required_call_args {
        if arguments.len() != needed_args as usize {
            return Err(anyhow!(format!(
                "Task \"{}\" requires {} arguments, but {} were provided",
                taskname,
                needed_args,
                arguments.len()
            )));
        }
    } else if !arguments.is_empty() {
        return Err(anyhow!(format!(
            "Task \"{}\" does not require any arguments, but {} were provided",
            taskname,
            arguments.len()
        )));
    }

    // Manually build taskcall
    let taskcall = TaskCall {
        task: taskname.to_string(),
        args: Some(arguments),
    };

    // Build `TaskExecutor` instance
    TaskExecutor::new(task, &taskcall, &work_dir)
}
