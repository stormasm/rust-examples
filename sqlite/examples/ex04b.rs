use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db = Connection::open("people.db")?;
    load_module(&db)?;
    let mut stmt = db.prepare("SELECT * FROM vtab")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let s1: Option<String> = row.get_unwrap(2);
        println!("{:?}", s1.unwrap());
    }

    Ok(())
}
