use anyhow::{anyhow, Result};
use std::{
    io,
    os::unix::process::ExitStatusExt,
    path::Path,
    process::{Child, Command, ExitStatus},
};

use crate::schema::{task::Task, task_call::TaskCall};

/// TaskExecutor is a struct that will be responsible for executing a single task.
pub struct TaskExecutor {
    pub task: Task,
    process: Command,
    child_process: Option<Child>,
}

impl TaskExecutor {
    /// Create a new TaskExecutor
    pub fn new(
        task: &Task,
        taskcall: &TaskCall,
        set_working_dir: &Option<String>,
    ) -> Result<TaskExecutor> {
        // Setup initial command
        let mut command: Command = Command::new(&task.command);

        // Add task arguments
        if let Some(args) = &task.args {
            args.iter().for_each(|arg| {
                command.arg(arg);
            });
        }

        // Add taskcall additional arguments and check amount of required args
        if let Some(args) = &taskcall.args {
            if let Some(required_call_args) = task.required_call_args {
                if args.len() != required_call_args as usize {
                    let error_message = format!(
                        "Task {} requires exactly {} additional arguments but {} were provided",
                        task.command,
                        required_call_args,
                        args.len()
                    );
                    return Err(anyhow!(error_message));
                }
            }

            command.args(args);
        } else if let Some(required_call_args) = task.required_call_args {
            if required_call_args > 0 {
                let error_message = format!(
                    "Task {} requires exactly {} additional arguments but 0 were provided",
                    task.command, required_call_args,
                );
                return Err(anyhow!(error_message));
            }
        }

        // Change working directory if needed
        if let Some(dir) = set_working_dir {
            let stem_path = Path::new(&dir).parent().unwrap();
            if stem_path.is_dir() {
                command.current_dir(stem_path);
            }
        }

        Ok(TaskExecutor {
            task: task.clone(),
            process: command,
            child_process: None,
        })
    }

    /// Executes the process and stores the child process
    pub fn execute(&mut self) -> Result<()> {
        let child = self.process.spawn();
        match child {
            Ok(child) => {
                self.child_process = Some(child);
                Ok(())
            }
            Err(error) => {
                let error_message = format!(
                    "Cannot spawn process for task '{}' ({})",
                    self.task.command, error
                );
                Err(anyhow!(error_message))
            }
        }
    }

    /// Waits for the child process to finish and returns the childs status code.
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        let child = self.child_process.as_mut().unwrap();
        child.wait()
    }

    /// Tries to wait for the child process to finish and returns the childs status code.
    /// This method does not actually wait for the child process to finish.
    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        let Some(child) = self.child_process.as_mut() else {
            return Ok(Some(ExitStatus::from_raw(1)));
        };

        child.try_wait()
    }
}
