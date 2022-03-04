use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Moravec {
    id: i32,
    entry: String,
}

fn main() -> Result<()> {
    let conn = Connection::open("mydb1.db")?;

    let mut stmt = conn.prepare("SELECT id, entry FROM moravec")?;
    let moravec_iter = stmt.query_map([], |row| {
        Ok(Moravec {
            id: row.get(0)?,
            entry: row.get(1)?,
        })
    })?;

    for moravec in moravec_iter {
        println!("Found moravec {:?}", moravec.unwrap());
    }
    Ok(())
}
