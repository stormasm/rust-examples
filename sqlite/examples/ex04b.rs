//use fallible_iterator::FallibleIterator;
use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db = Connection::open("people.db")?;
    load_module(&db)?;
    let mut stmt = db.prepare("SELECT * FROM vtab")?;

    //  let rows: Rows<'_> = stmt.query([])?;
    let mut rows = stmt.query([])?;

    //while let Some(row) = rows.next()? {
    //println!("{:?}", row.unwrap();

    let row1 = rows.next()?.unwrap();
    let s1: Option<String> = row1.get_unwrap(2);
    println!("{:?}", s1.unwrap());
    //}

    Ok(())
}
