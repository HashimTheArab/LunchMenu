use select::document::Document;
use select::predicate::{Attr, Class};
use prettytable::{Table, row, cell, Cell, format};
use std::io;
use std::io::Write;

fn main() {
    print!("Select an option:\n[1] Breakfast\n[2] Lunch\n[?]: ");
    io::stdout().flush().expect("Error flushing stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");

    match input.trim() {
        "1" => print_menu(String::from("Breakfast")),
        "2" => print_menu(String::from("Lunch")),
        _ => {
            println!("That wasn't an option.");
            main();
        }
    };
}

fn print_menu(meal_type: String) {
    let html = reqwest::blocking::get("").unwrap(); // if you goto my school just put the lunch menu url inside the quotes
    let mut week: usize = 0;
    let mut table: Table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]);

    Document::from_read(html)
        .unwrap()
        .select(Attr("data-testid", "dining-menu-item"))
        .for_each(|e| {
            let date: String = e.select(Class("date"))
                .next()
                .unwrap()
                .text()
                .trim()
                .parse()
                .unwrap();
            if date.contains("Monday") {
                week += 1;
                table.add_empty_row();
            }
            let mut cell_data: String = date.split(',').last().unwrap().to_string() + "\n";
            cell_data += e.select(Class(meal_type.to_lowercase().as_str()))
                .next()
                .unwrap()
                .text()
                .replacen(format!("{} ", meal_type).as_str(), "", 1)
                .as_str();
            table.get_mut_row(week).unwrap().add_cell(Cell::new(cell_data.as_str()));
        }
    );
    table.printstd();
}
