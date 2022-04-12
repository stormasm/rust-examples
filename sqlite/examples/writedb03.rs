use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open("person03.db")?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;

    let me0 = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };

    let me1 = Person {
        id: 1,
        name: "Pete".to_string(),
        data: Some(r#"a"#.as_bytes().to_vec()),
    };

    let me2 = Person {
        id: 2,
        name: "Bill".to_string(),
        data: None,
    };

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me0.name, me0.data],
    )?;

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me1.name, me1.data],
    )?;

    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me2.name, me2.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
