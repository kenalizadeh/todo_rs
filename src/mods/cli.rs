use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn command(&self) -> &Commands {
        &self.command
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add new task
    Add {
        /// Task summary
        #[arg(short, long)]
        summary: String,
    },
    /// Delete task with id
    Delete {
        /// Task id to delete
        #[arg(short, long)]
        id: usize,
    },
    /// Update task with id
    Update {
        /// New summary
        #[arg(short, long)]
        summary: String,

        /// Task id to update
        #[arg(short, long)]
        id: usize,
    },
    /// Toggle task done status
    Done {
        /// Task id to mark done/undone
        #[arg(short, long)]
        id: usize,
    },
    /// List all tasks
    List {
        /// Filter out done tasks
        #[arg(short, long)]
        no_done: bool,
    },
}
