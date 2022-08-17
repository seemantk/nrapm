extern crate log;
use term_table::table_cell::TableCell;
use term_table::row::Row;
use crate::cmd;
use crate::cmd::nrkv;

pub fn process_get(c: &cmd::Cli) {
    log::trace!("NRAPM Get() reached");
    let mut table = term_table::Table::new();
    table.max_column_width = 40;
    table.style = term_table::TableStyle::extended();
    table.add_row(Row::new(vec![
        TableCell::new("Key"),
        TableCell::new("Value")
    ]));
    let values = nrkv::values(&c);
    for (key, value) in &values {
        table.add_row(Row::new(vec![
            TableCell::new(key),
            TableCell::new(value.to_string())
        ]));
    }
    println!("{}", table.render());
}
