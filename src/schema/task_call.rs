use serde::{Deserialize, Serialize};

/// Represents a `TaskCall` from a configuration file
#[derive(Serialize, Deserialize, Clone)]
pub struct TaskCall {
    pub task: String,
    pub args: Option<Vec<String>>,
}
