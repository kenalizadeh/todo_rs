mod mods;

use crate::mods::cli::Cli;
use crate::mods::table::print_table;
use clap::Parser;
use mods::parse::parse_command;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let items = parse_command(cli.command()).await.unwrap();
    print_table(items);
}
