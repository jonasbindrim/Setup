use anyhow::{anyhow, Result};

use crate::{
    schema::project::Project,
    task_executor::TaskExecutor,
    util::{format_error, import_project_value, print_message, MessageSeverity},
};

/// Executes run job mode
pub fn run_job_mode(projectfile: String, job: String) -> Result<()> {
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

    // Execute job
    print_message(MessageSeverity::Info, format!("Executing job \"{}\"", job));
    match execute_job(&project, &job, work_dir) {
        Ok(()) => {
            print_message(
                MessageSeverity::Success,
                format!("Job \"{}\" executed successfully", &job),
            );
            Ok(())
        }
        Err(error) => {
            eprintln!("{}", format_error(format!("{}", error)));
            Err(anyhow!(format!("Job \"{}\" failed", &job)))
        }
    }
}

/// Executes the job with the given name and therefore all tasks associated with that job
fn execute_job(project: &Project, jobname: &str, work_dir: Option<String>) -> Result<()> {
    // Get the tasknames associated with the job
    let Some(job) = project.jobs.get(jobname) else {
        return Err(anyhow!(format!("Job with name \"{}\" not found", jobname)));
    };

    // Build `TaskExecutor` instances for each task
    let mut task_executors: Vec<TaskExecutor> = Vec::new();
    for taskcall in job.tasks.iter() {
        let Some(task) = project.tasks.get(&taskcall.task) else {
            return Err(anyhow!(format!(
                "Task with name \"{}\" not found",
                taskcall.task
            )));
        };

        task_executors.push(TaskExecutor::new(task, taskcall, &work_dir)?);
    }

    // Call executors functions
    match job.parallel {
        Some(true) => execute_parallel(task_executors),
        _ => execute_sequential(task_executors),
    }
}

/// Executes multiple tasks sequentially
fn execute_sequential(task_executors: Vec<TaskExecutor>) -> Result<()> {
    // Execute each `TaskExecutor` and wait for it to finish
    for mut executor in task_executors {
        executor.execute()?;

        if !executor.wait().unwrap().success() {
            return Err(anyhow!(format!(
                "Task \"{}\" failed",
                executor.task.command
            )));
        } else {
            print_message(
                MessageSeverity::Success,
                format!(
                    "Task \"{}\" executed successfully",
                    executor.execution_string
                ),
            );
        }
    }

    Ok(())
}

/// Executes multiple tasks in parallel
fn execute_parallel(mut task_executors: Vec<TaskExecutor>) -> Result<()> {
    // Store task status
    let mut task_status: Vec<bool> = vec![false; task_executors.len()];
    let mut task_failed = false;

    // Execute each `TaskExecutor` without waiting for it to finish
    for executor in &mut task_executors {
        executor.execute()?;
    }

    // Wait for all tasks to finish
    loop {
        let mut all_finished = true;

        for (index, executor) in task_executors.iter_mut().enumerate() {
            if !task_status[index] {
                all_finished = false;
                match executor.try_wait() {
                    Ok(statuscode) => {
                        if let Some(statuscode) = statuscode {
                            task_status[index] = true;
                            if !statuscode.success() {
                                task_failed = true;
                                eprintln!(
                                    "{}",
                                    format_error(format!(
                                        "Task \"{}\" failed",
                                        executor.task.command
                                    ))
                                );
                            } else {
                                print_message(
                                    MessageSeverity::Success,
                                    format!(
                                        "Task \"{}\" executed successfully",
                                        executor.task.command
                                    ),
                                );
                            }
                        }
                    }
                    Err(_) => eprintln!(
                        "{}",
                        format_error(String::from("Something unexpected happened"))
                    ),
                }
            }
        }

        if all_finished {
            break;
        }
    }

    if task_failed {
        let error_message = String::from("Atleast one task failed to finished successfully");
        Err(anyhow!(error_message))
    } else {
        Ok(())
    }
}
