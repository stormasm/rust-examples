use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    let sql = "SELECT * \
    FROM table_1";

    let dialect = GenericDialect {};

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("AST: {:?}", ast);
}
