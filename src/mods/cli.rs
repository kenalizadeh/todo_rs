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
    /// Mark task done
    Done {
        /// Task id to mark done
        #[arg(short, long)]
        id: usize,
        /// Toggles if already done (default `false`)
        #[arg(short, long, default_value = "false")]
        toggle: bool,
    },
    /// List tasks
    List {
        /// Show done tasks as well (default `false`)
        #[arg(short, long, default_value = "false")]
        all: bool,
    },
}
