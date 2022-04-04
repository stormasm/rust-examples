use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db = Connection::open("people.db")?;
    load_module(&db)?;
    let mut stmt = db.prepare("SELECT * FROM vtab")?;
    let num_of_columns = stmt.column_count();
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        for n in 0..num_of_columns {
            let s1: Option<String> = row.get_unwrap(n);
            print!("{} ", s1.unwrap());
        }
        println!("");
    }

    Ok(())
}
