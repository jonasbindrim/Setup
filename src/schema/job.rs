use serde::{Deserialize, Serialize};

use super::task_call::TaskCall;

/// Represents a `Job` from a configuration file
#[derive(Serialize, Deserialize, Clone)]
pub struct Job {
    pub tasks: Vec<TaskCall>,
    pub parallel: Option<bool>,
}
