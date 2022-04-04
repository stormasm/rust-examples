use rusqlite::vtab::csvtab::load_module;
use rusqlite::{Connection, Result};
use std::env::args;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("Please enter a database name and a csv file name like this:");
        println!("people1.db ./csv/people1.csv");
    } else {
        let dbname = &args[1];
        let myfilename = &args[2];
        let db = Connection::open(dbname)?;
        load_module(&db)?;

        let s = format!(
            "CREATE VIRTUAL TABLE vtab USING csv(filename={}, header=yes)",
            myfilename
        );

        let _x = db.execute_batch(&s);
    }

    Ok(())
}
