use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};
use std::env::args;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("Please enter a tablename and a database name:");
        println!("mytable mydatabase.db");
    } else {
        let tablename = &args[1];
        let dbname = &args[2];
        let db = Connection::open(dbname)?;

        let s = format!("SELECT * FROM {}", tablename);

        load_module(&db)?;

        let mut stmt = db.prepare(&s)?;
        let num_of_columns = stmt.column_count();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            for n in 0..num_of_columns {
                let s1: Option<String> = row.get_unwrap(n);
                print!("{} ", s1.unwrap());
            }
            println!("");
        }
    }
    Ok(())
}
