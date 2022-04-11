use nu_protocol::Value;
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result, Row};

fn convert_sqlite_value_to_nu_value(value: ValueRef) -> Value {
    match value {
        ValueRef::Null => {
            println!("got Null")
        }
        ValueRef::Integer(i) => println!("got Integer {:?}", i),
        ValueRef::Real(f) => {
            println!("got Real {:?}", f)
        }
        ValueRef::Text(s) => {
            println!("got Text {:?}", s)
        }
        ValueRef::Blob(u) => println!("got Blob {:?}", u),
    }
}

fn convert_sqlite_row_to_nu_value(row: &Row) -> Value {
    let mut collected = TaggedDictBuilder::new(tag.clone());
    for (i, c) in row.as_ref().column_names().iter().enumerate() {
        collected.insert_value(
            c.to_string(),
            convert_sqlite_value_to_nu_value(row.get_ref_unwrap(i)),
        );
    }
    collected.into_value()
}

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
            let mut out = Vec::new();
            println!("{:?}", table_name);
            let mut table_stmt = conn.prepare(&format!("select * from [{}]", table_name))?;
            let mut table_rows = table_stmt.query([])?;
            while let Some(table_row) = table_rows.next()? {
                println!("hola");
                out.push(convert_sqlite_row_to_nu_value(table_row))
            }
        }
    }

    Ok(())
}
