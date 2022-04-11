use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    // to create this db run writedb01
    let conn = Connection::open("person01.db")?;

    let mut meta_stmt = conn.prepare("select name from sqlite_master where type='table'")?;
    let mut meta_rows = meta_stmt.query([])?;

    let tables = vec!["person".to_string()];

    while let Some(meta_row) = meta_rows.next()? {
        let table_name: String = meta_row.get(0)?;
        println!("{:?}", table_name);
        if tables.is_empty() || tables.contains(&table_name) {
            println!("{:?}", table_name);
            let mut table_stmt = conn.prepare(&format!("select * from [{}]", table_name))?;
            let mut table_rows = table_stmt.query([])?;
            while let Some(_table_row) = table_rows.next()? {
                println!("hola");
                // out.push(convert_sqlite_row_to_nu_value(table_row, tag.clone()))
            }
        }
    }

    Ok(())
}
