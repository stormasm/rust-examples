use nu_protocol::{Span, Value};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result, Row};

fn convert_sqlite_value_to_nu_value(value: ValueRef) -> Value {
    let span = Span::new(0, 0);

    match value {
        ValueRef::Null => {
            //println!("got Null");
            Value::Nothing { span }
        }
        ValueRef::Integer(i) => {
            //println!("got Integer {:?}", i);
            Value::Int { val: i, span: span }
        }
        ValueRef::Real(f) => {
            //println!("got Real {:?}", f);
            Value::Float { val: f, span: span }
        }
        ValueRef::Text(buf) => {
            let s = match std::str::from_utf8(buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            //println!("got Text {:?}", s.to_string());
            Value::String {
                val: s.to_string(),
                span: span,
            }
        }
        ValueRef::Blob(u) => {
            //println!("got Blob {:?}", u);
            Value::Binary {
                val: u.to_vec(),
                span: span,
            }
        }
    }
}

fn convert_sqlite_row_to_nu_value(row: &Row) -> Value {
    let mut vals = Vec::new();
    let colnamestr = row.as_ref().column_names().to_vec();
    // convert str to String
    let colnames = colnamestr.iter().map(|s| s.to_string()).collect();

    for (i, c) in row.as_ref().column_names().iter().enumerate() {
        let _column = c.to_string();
        let val = convert_sqlite_value_to_nu_value(row.get_ref_unwrap(i));
        vals.push(val);
    }

    Value::Record {
        cols: colnames,
        vals: vals,
        span: Span::test_data(),
    }
}

fn main() -> Result<()> {
    // to create this db run writedb04
    let conn = Connection::open("person04.db")?;

    let mut meta_stmt = conn.prepare("select name from sqlite_master where type='table'")?;
    let mut meta_rows = meta_stmt.query([])?;

    let tables = vec!["person".to_string()];

    while let Some(meta_row) = meta_rows.next()? {
        let table_name: String = meta_row.get(0)?;
        // println!("table name = {:?}", table_name);
        if tables.is_empty() || tables.contains(&table_name) {
            let mut out = Vec::new();
            //println!("table name = {:?}", table_name);
            let mut table_stmt = conn.prepare(&format!("select * from [{}]", table_name))?;
            let mut table_rows = table_stmt.query([])?;
            while let Some(table_row) = table_rows.next()? {
                out.push(convert_sqlite_row_to_nu_value(table_row))
            }
            //println!("{:?}", out);
            let _list = Value::List {
                vals: out,
                span: Span::test_data(),
            };
            //println!("{:?}", list);
        }
    }

    Ok(())
}
