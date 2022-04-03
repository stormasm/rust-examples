use rusqlite::{Connection, Result};

//use rusqlite::vtab::{VTabConnection, VTabCursor, Values};

/*
use rusqlite::vtab::{
    dequote, escape_double_quote, parse_boolean, read_only_module, Context, CreateVTab, IndexInfo,
    VTab, VTabConnection, VTabCursor, Values,
};
*/
//use crate::{Connection, Error, Result};
use fallible_iterator::FallibleIterator;
use rusqlite::vtab::csvtab::load_module;

fn main() -> Result<()> {
    let db = Connection::open("mydb3.db")?;

    //let db = Connection::open_in_memory()?;
    load_module(&db)?;
    db.execute_batch("CREATE VIRTUAL TABLE vtab USING csv(filename='./csv/test.csv', header=yes)")?;

    {
        let mut s = db.prepare("SELECT rowid, * FROM vtab")?;
        {
            let headers = s.column_names();
            assert_eq!(vec!["rowid", "colA", "colB", "colC"], headers);
        }

        let ids: Result<Vec<i32>> = s.query([])?.map(|row| row.get::<_, i32>(0)).collect();
        let sum = ids?.iter().sum::<i32>();
        assert_eq!(sum, 15);
    }
    //db.execute_batch("DROP TABLE vtab");

    Ok(())
}
