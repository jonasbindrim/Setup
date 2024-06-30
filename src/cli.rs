use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliParameters {
    /// Path to the project file. If ommited, `setup` tries to auto detect a project file.
    pub projectfile: Option<String>,

    /// Flag to run in silent mode. Supresses all output from child processes.
    #[clap(short, long)]
    pub silent_children: Option<bool>,

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
        /// Name of the task to execute
        task: String,

        /// Additional arguments for the task
        arguments: Vec<String>,

    },
    /// Lists all available jobs in the project
    ListJobs,
    /// Lists all available tasks in the project
    ListTasks,
}
