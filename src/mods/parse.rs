use std::env::Args;
use std::process;
use sqlx::Error;
use crate::mods::todo::Todo;
use crate::mods::db;

pub async fn parse_args(args: Args) -> Result<Vec<Todo>, Error> {
    let mut iter = args.into_iter();
    _ = iter.next();
    let _arg = iter.next().unwrap();

    db::migrations().await?;

    match _arg.as_str() {
        "add" => {
            let content = iter.next().unwrap_or_else(|| {
                eprintln!("todo content missing");
                process::exit(1);
            });

            db::add(content.as_str()).await?;
        },
        "delete" => {
            let id = iter.next().unwrap_or_else(|| {
                eprintln!("ID is missing");
                process::exit(1);
            });

            let id: u8 = id.parse().unwrap();

            db::delete(id).await?;
        },
        "done" => {
            let id = iter.next().unwrap_or_else(|| {
                eprintln!("ID is missing");
                process::exit(1);
            });

            let id: u8 = id.parse().unwrap();

            if let Some(arg) = iter.next() {
                let done: bool = arg.parse().unwrap_or_else(|_| {
                    eprintln!("Invalid argument; use true | false");
                    process::exit(1);
                });
                db::mark_done(id, done).await?;
            } else {
                db::mark_done(id, true).await?;
            }
        },
        "update" => {
            let id = iter.next().unwrap_or_else(|| {
                eprintln!("ID is missing");
                process::exit(1);
            });

            let id: u8 = id.parse().unwrap();

            let content = iter.next().unwrap_or_else(|| {
                eprintln!("todo content missing");
                process::exit(1);
            });

            db::update(id, content.as_str()).await?;
        }
        "list" => {
            return db::list().await
        },
        _ => {
            eprintln!("arguments are missing");
            process::exit(1);
        }
    }

    return db::list().await
}