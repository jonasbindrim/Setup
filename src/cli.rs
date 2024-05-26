use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliParameters {
    /// Path to the project file
    pub projectfile: String,
    /// Name of the job to execute
    pub job: String,
}