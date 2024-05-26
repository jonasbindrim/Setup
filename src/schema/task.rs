use serde::{Deserialize, Serialize};

/// Represents a `Task` from a configuration file
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub required_call_args: Option<u8>,
}
