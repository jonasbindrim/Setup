use anyhow::{anyhow, Result};
use std::{
    io::{self, BufRead, BufReader},
    os::unix::process::ExitStatusExt,
    path::Path,
    process::{Child, Command, ExitStatus, Stdio},
    thread::{self, JoinHandle},
};

use crate::{
    schema::{task::Task, task_call::TaskCall},
    util::{print_message, MessageSeverity},
};

/// TaskExecutor is a struct that will be responsible for executing a single task.
pub struct TaskExecutor {
    pub task: Task,
    pub execution_string: String,
    process: Command,
    child_process: Option<Child>,
    err_reader_handle: Option<JoinHandle<()>>,
    out_reader_handle: Option<JoinHandle<()>>,
}

impl TaskExecutor {
    /// Create a new TaskExecutor
    pub fn new(
        task: &Task,
        taskcall: &TaskCall,
        set_working_dir: &Option<String>,
    ) -> Result<TaskExecutor> {
        let mut execution_command = String::new();

        // Setup initial command
        let mut command: Command = Command::new(&task.command);
        execution_command.push_str(&task.command);

        // Add task arguments
        if let Some(args) = &task.args {
            command.args(args);
            args.iter().for_each(|arg| {
                execution_command.push_str(format!(" {}", arg).as_str());
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
            args.iter().for_each(|arg| {
                execution_command.push_str(format!(" {}", arg).as_str());
            });
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

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        Ok(TaskExecutor {
            task: task.clone(),
            process: command,
            child_process: None,
            execution_string: execution_command,
            err_reader_handle: None,
            out_reader_handle: None,
        })
    }

    /// Executes the process and stores the child process
    pub fn execute(&mut self) -> Result<()> {
        let child = self.process.spawn();
        match child {
            Ok(mut child) => {
                print_message(
                    MessageSeverity::Info,
                    format!("Executing task \"{}\"...", self.execution_string),
                );

                self.bind_output(&mut child);
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

    /// Binds stdout and stderr to the given output and prefixes both with the given prefix.
    fn bind_output(&mut self, child: &mut Child) {
        let stdout = child.stdout.take().expect("Unable to take child stdout");
        let stderr = child.stderr.take().expect("Unable to take child stderr");

        let out_reader = BufReader::new(stdout);
        let err_reader = BufReader::new(stderr);

        let out_reader_execution_string = self.execution_string.clone();
        self.out_reader_handle = Some(thread::spawn(move || {
            for line in out_reader.lines().map_while(Result::ok) {
                print_message(
                    MessageSeverity::ChildInfo,
                    format!("{} -> {}", out_reader_execution_string, line),
                );
            }
        }));

        let err_reader_execution_string = self.execution_string.clone();
        self.err_reader_handle = Some(thread::spawn(move || {
            for line in err_reader.lines().map_while(Result::ok) {
                print_message(
                    MessageSeverity::ChildError,
                    format!("{} -> {}", err_reader_execution_string, line),
                );
            }
        }));
    }

    /// Waits for the child process to finish and returns the childs status code.
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        let child = self.child_process.as_mut().unwrap();
        let exitstate = child.wait()?;
        self.out_reader_handle.take().unwrap().join().unwrap();
        self.err_reader_handle.take().unwrap().join().unwrap();

        Ok(exitstate)
    }

    /// Tries to wait for the child process to finish and returns the childs status code.
    /// This method does not actually wait for the child process to finish.
    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        let Some(child) = self.child_process.as_mut() else {
            return Ok(Some(ExitStatus::from_raw(1)));
        };

        let exitstatus = child.try_wait()?;
        if exitstatus.is_some() {
            self.out_reader_handle.take().unwrap().join().unwrap();
            self.err_reader_handle.take().unwrap().join().unwrap();
        }

        Ok(exitstatus)
    }
}
