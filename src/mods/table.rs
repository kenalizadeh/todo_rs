use std::fmt::Display;
use crate::mods::todo;
use ascii_table::{Align,AsciiTable};

pub fn print_table(items: Vec<todo::Todo>) {

    let mut ascii_table = AsciiTable::default();
    ascii_table.column(0).set_header("ID").set_align(Align::Center);
    ascii_table.column(1).set_header("Todo");
    ascii_table.column(2).set_header("Done").set_align(Align::Center);
    ascii_table.column(3).set_header("Created At");
    ascii_table.column(4).set_header("Done At");

    if items.is_empty() {
        println!("Todo list is empty!")
    } else {
        let data: Vec<Vec<&dyn Display>> = items.iter().map(|t| {
            todo::map_display(t)
        }).collect();

        ascii_table.print(data);
    }
}