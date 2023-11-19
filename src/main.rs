mod tests;
mod mods;

use std::env;
use mods::parse::parse_args;
use crate::mods::table::print_table;

#[tokio::main]
async fn main() {
    let args = env::args();
    let items = parse_args(args).await.unwrap();
    print_table(items);
}