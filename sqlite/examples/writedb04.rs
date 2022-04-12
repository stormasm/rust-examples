use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
    grade: f64,
}

fn main() -> Result<()> {
    let conn = Connection::open("person04.db")?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT,
                  data            BLOB,
                  grade           REAL
                  )",
        [],
    )?;

    let me0 = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
        grade: 3.4,
    };

    let me1 = Person {
        id: 1,
        name: "Pete".to_string(),
        data: Some(r#"a"#.as_bytes().to_vec()),
        grade: 4.1,
    };

    let me2 = Person {
        id: 2,
        name: "Bill".to_string(),
        data: None,
        grade: 5.2,
    };

    let me3 = Person {
        id: 3,
        name: "".to_string(),
        data: None,
        grade: 8.2,
    };

    conn.execute(
        "INSERT INTO person (name, data, grade) VALUES (?1, ?2, ?3)",
        params![me0.name, me0.data, me0.grade],
    )?;

    conn.execute(
        "INSERT INTO person (name, data, grade) VALUES (?1, ?2, ?3)",
        params![me1.name, me1.data, me1.grade],
    )?;

    conn.execute(
        "INSERT INTO person (name, data, grade) VALUES (?1, ?2, $3)",
        params![me2.name, me2.data, me2.grade],
    )?;

    conn.execute(
        "INSERT INTO person (name, data, grade) VALUES (?1, ?2, $3)",
        params![me3.name, me3.data, me3.grade],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data, grade FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
            grade: row.get(3)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
