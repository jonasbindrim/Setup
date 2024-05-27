use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{task_executor::TaskExecutor, JSONSCHEMA};

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
        serde_json::from_value::<Project>(project_data)
            .unwrap_or_else(|error| panic!("Error deserializing JSON: {}", error))
    }

    /// Validates a `Project` from a JSON string. Panics if the project is invalid.
    pub fn validate_project(project: &Value) {
        let schema = JSONSCHEMA.get().unwrap();

        if !schema.is_valid(project) {
            panic!("Project data does not match the json schema");
        }
    }

    /// Executes the job with the given name and therefore all tasks associated with that job
    pub fn execute_job(&self, jobname: &str) {
        // Get the tasknames associated with the job
        let Some(taskcalls) = self.jobs.get(jobname) else {
            panic!("Job with name {} not found", jobname);
        };

        // Execute each task individually
        for taskcall in taskcalls.iter() {
            let Some(task) = self.tasks.get(&taskcall.task) else {
                panic!("Task with name {} not found", taskcall.task)
            };

            let mut task_executor = TaskExecutor::new(task, taskcall);
            task_executor
                .execute()
                .unwrap_or_else(|err| panic!("Error executing task: {}", err));
            if !task_executor.wait().unwrap().success() {
                panic!("Task {} failed", task.command);
            }
        }
    }
}
