use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "SELECT * FROM Glue WHERE id > 100;",
    ];
    
    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}