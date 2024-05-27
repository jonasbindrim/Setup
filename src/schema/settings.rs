use serde::{Deserialize, Serialize};

/// Represents the settings from a configuration file
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub project_file_as_work_dir: Option<bool>,
}
