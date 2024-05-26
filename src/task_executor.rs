use anyhow::Result;
use std::process::{Child, Command, ExitStatus};

use crate::{
    errors::ExecutionError,
    schema::{task::Task, task_call::TaskCall},
};

/// TaskExecutor is a struct that will be responsible for executing a single task.
pub struct TaskExecutor {
    task: Task,
    process: Command,
    child_process: Option<Child>,
}

impl TaskExecutor {
    /// Create a new TaskExecutor
    pub fn new(task: &Task, taskcall: &TaskCall) -> TaskExecutor {
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
                    panic!(
                        "Task {} requires exactly {} additional arguments but {} were provided",
                        task.command,
                        required_call_args,
                        args.len()
                    );
                }
            }

            args.iter().for_each(|arg| {
                command.arg(arg);
            });
        } else if let Some(required_call_args) = task.required_call_args {
            if required_call_args > 0 {
                panic!(
                    "Task {} requires exactly {} additional arguments but 0 were provided",
                    task.command, required_call_args
                );
            }
        }

        TaskExecutor {
            task: task.clone(),
            process: command,
            child_process: None,
        }
    }

    /// Executes the process and stores the child process
    pub fn execute(&mut self) -> Result<()> {
        let Ok(child) = self.process.spawn() else {
            return Err(ExecutionError::SpawnError {
                command: self.task.command.to_string(),
            }
            .into());
        };

        self.child_process = Some(child);
        Ok(())
    }

    /// Waits for the child process to finish and returns the childs status code.
    pub fn wait(&mut self) -> Result<ExitStatus> {
        let child = self.child_process.as_mut().unwrap();
        Ok(child.wait()?)
    }
}
