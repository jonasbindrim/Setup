use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliParameters {
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Check whether a given project file is valid in regards to the schema
    Validate {
        /// The path to the project file
        projectfile: String, 
    },
    /// Execute a job from a project file
    Run {
        /// Path to the project file
        projectfile: String,
        /// Name of the job to execute
        job: String,
    }
}