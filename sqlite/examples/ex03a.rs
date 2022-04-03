use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db = Connection::open("mydb3a.db")?;
    load_module(&db)?;
    db.execute_batch("CREATE VIRTUAL TABLE vtab USING csv(filename='./csv/test.csv', header=yes)")?;
    Ok(())
}
