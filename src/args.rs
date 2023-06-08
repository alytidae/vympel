use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AppArgs {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(skip)]
    Show,
    /// Add task 
    Add{
        task_body: String, 
        task_priority: Option<TaskPriority>,
    },
    /// Remove task
    Rm {
        /// Number of task to remove
        task_number: u32,
    },
    /// Show completed and incomplete tasks
    All,
    // TODO: Edit(u32),
}

#[derive(Debug, ValueEnum, Clone)]
pub enum TaskPriority {
    Low,
    /// Default value
    Mid,
    High,
}
