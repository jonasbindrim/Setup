use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliParameters {
    /// Path to the project file. If ommited, `setup` tries to auto detect a project file.
    pub projectfile: Option<String>,

    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Check whether the project is valid in regards to the schema
    Validate,
    /// Execute a job from the project
    Run {
        /// Name of the job to execute
        job: String,
    },
    /// Execute a single task from the project
    RunTask {
        task: String,
        arguments: Vec<String>,
    },
    /// Lists all available jobs in the project
    ListJobs,
    /// Lists all available tasks in the project
    ListTasks,
}
