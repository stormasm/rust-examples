use std::env::args;

use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("Please enter a database name and a csv file name");
    } else {
        let dbname = &args[1];
        let myfilename = &args[2];

        // let db = Connection::open("mydb3a.db")?;
        let db = Connection::open(dbname)?;

        load_module(&db)?;

        //    let s = format!("{}", foo)

        let s = format!(
            "CREATE VIRTUAL TABLE vtab USING csv(filename={}, header=yes)",
            myfilename
        );

        //db.execute_batch("CREATE VIRTUAL TABLE vtab USING csv(filename='./csv/test.csv', header=yes)")?;
        let _x = db.execute_batch(&s);
    }

    Ok(())
}
