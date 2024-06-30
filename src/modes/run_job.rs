use anyhow::{anyhow, Error, Result};

use crate::{
    schema::{job::Job, project::Project},
    task_executor::TaskExecutor,
    util::{format_error, import_project_value, print_message, MessageSeverity},
};

/// Executes the `run` mode
pub fn run_job_mode(projectfile: String, jobname: String, silent_children: bool) -> Result<()> {
    let job_execution = JobExecution::new(projectfile, jobname, silent_children)?;
    job_execution.execute()
}

struct JobExecution {
    jobname: String,
    job: Job,
    task_executors: Vec<TaskExecutor>,
}

impl JobExecution {
    /// Creates a new `JobExecution` instance
    fn new(projectfile: String, jobname: String, silent_children: bool) -> Result<JobExecution> {
        // Import project file
        let project_data = import_project_value(&projectfile)?;
        let project = Project::import_project(project_data)?;

        // Get the tasknames associated with the job
        let Some(job) = project.jobs.get(&jobname) else {
            return Err(anyhow!(format!("Job with name \"{}\" not found", &jobname)));
        };
        let job = job.clone();

        // Check working dir
        let work_dir: Option<String> = {
            if let Some(settings) = &project.settings {
                match settings.project_file_as_work_dir {
                    Some(true) => Some(projectfile.clone()),
                    _ => None,
                }
            } else {
                None
            }
        };

        // Build `TaskExecutor` instances for each task
        let mut task_executors: Vec<TaskExecutor> = Vec::new();
        for taskcall in job.tasks.iter() {
            let Some(task) = project.tasks.get(&taskcall.task) else {
                return Err(Self::execution_error_handler(
                    &anyhow!(format!("Task with name \"{}\" not found", taskcall.task)),
                    &jobname,
                ));
            };

            task_executors.push(TaskExecutor::new(
                task,
                taskcall,
                &work_dir,
                silent_children,
            )?);
        }

        Ok(JobExecution {
            jobname,
            job,
            task_executors,
        })
    }

    /// Executes the job
    fn execute(self) -> Result<()> {
        let jobname = self.jobname.clone();
        print_message(
            MessageSeverity::Info,
            format!("Executing job \"{}\"", &jobname),
        );

        // Call executors functions
        let execution_result = match self.job.parallel {
            Some(true) => self.execute_parallel(),
            _ => self.execute_sequential(),
        };

        // Handle execution result
        match execution_result {
            Ok(()) => {
                print_message(
                    MessageSeverity::Success,
                    format!("Job \"{}\" executed successfully", &jobname),
                );
                Ok(())
            }
            Err(error) => Err(Self::execution_error_handler(&error, &jobname)),
        }
    }

    /// Prints the given error and returns a execution failed error
    fn execution_error_handler(error: &Error, jobname: &str) -> anyhow::Error {
        eprintln!("{}", format_error(format!("{}", error)));
        anyhow!(format!("Job \"{}\" failed", jobname))
    }

    /// Executes multiple tasks sequentially
    fn execute_sequential(mut self) -> Result<()> {
        // Execute each `TaskExecutor` and wait for it to finish
        for executor in &mut self.task_executors {
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
    fn execute_parallel(mut self) -> Result<()> {
        // Store task status
        let mut task_status: Vec<bool> = vec![false; self.task_executors.len()];
        let mut task_failed = false;

        // Execute each `TaskExecutor` without waiting for it to finish
        for executor in &mut self.task_executors {
            executor.execute()?;
        }

        // Wait for all tasks to finish
        loop {
            let mut all_finished = true;

            for (index, executor) in self.task_executors.iter_mut().enumerate() {
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
}
