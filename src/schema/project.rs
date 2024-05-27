use std::{collections::HashMap, process::exit};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{task_executor::TaskExecutor, util::print_message, util::MessageSeverity, JSONSCHEMA};

use super::{task::Task, task_call::TaskCall};

type Job = Vec<TaskCall>;

/// Represents the content of a `Project` from a configuration file
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub jobs: HashMap<String, Job>,
    pub tasks: HashMap<String, Task>,
}

impl Project {
    /// Imports a `Project` from a JSON string
    pub fn import_project(project_data: Value) -> Project {
        // Validate project file against jsonschema
        Self::validate_project(&project_data);

        // Convert project_data to Project
        serde_json::from_value::<Project>(project_data).unwrap_or_else(|error| {
            print_message(
                MessageSeverity::Error,
                format!("Error deserializing JSON \"{}\"", error),
            );
            exit(1);
        })
    }

    /// Validates a `Project` from a JSON string. Panics if the project is invalid.
    pub fn validate_project(project: &Value) {
        let schema = JSONSCHEMA.get().unwrap();

        if !schema.is_valid(project) {
            print_message(
                MessageSeverity::Error,
                String::from("Project data from does not match the json schema"),
            );
            exit(1);
        }
    }

    /// Executes the job with the given name and therefore all tasks associated with that job
    pub fn execute_job(&self, jobname: &str) {
        // Get the tasknames associated with the job
        let Some(taskcalls) = self.jobs.get(jobname) else {
            print_message(
                MessageSeverity::Error,
                format!("Job with name \"{}\" not found", jobname),
            );
            exit(1);
        };

        // Execute each task individually
        for taskcall in taskcalls.iter() {
            let Some(task) = self.tasks.get(&taskcall.task) else {
                print_message(
                    MessageSeverity::Error,
                    format!("Task with name \"{}\" not found", taskcall.task),
                );
                exit(1);
            };

            let mut task_executor = TaskExecutor::new(task, taskcall);
            task_executor.execute().unwrap_or_else(|err| {
                print_message(
                    MessageSeverity::Error,
                    format!("Error executing task \"{}\"", err),
                );
                exit(1);
            });

            if !task_executor.wait().unwrap().success() {
                print_message(
                    MessageSeverity::Error,
                    format!("Task \"{}\" failed", task.command),
                );
                exit(1);
            }
        }
    }
}
