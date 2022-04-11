use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // to create this db run writedb01
    let conn = Connection::open("person01.db")?;

    let table_name: String = "person".to_string();

    //    let mut out = Vec::new();
    let mut table_stmt = conn.prepare(&format!("select * from [{}]", table_name))?;
    let mut table_rows = table_stmt.query([])?;
    while let Some(table_row) = table_rows.next()? {
        println!("hola");
        //println!("{:?}",table_row);
        // out.push(convert_sqlite_row_to_nu_value(table_row, tag.clone()))
    }
    Ok(())
}
