use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Failed to spawn process with command \"{command}\"")]
    SpawnError { command: String },
}
