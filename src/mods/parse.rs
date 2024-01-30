use super::cli::Commands;
use crate::mods::db;
use crate::mods::todo::Todo;
use sqlx::Error;

pub async fn parse_command(command: &Commands) -> Result<Vec<Todo>, Error> {
    db::migrations().await?;

    match command {
        Commands::Add { summary } => {
            db::add(summary).await?;
        }
        Commands::Delete { id } => {
            db::delete(*id).await?;
        }
        Commands::Update { summary, id } => {
            db::update(*id, summary).await?;
        }
        Commands::Done { id } => {
            db::toggle_done(*id).await?;
        }
        Commands::List { no_done } => return db::list(*no_done).await,
    }

    db::list(false).await
}
