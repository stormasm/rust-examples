use predicates::prelude::*;

const SQL_PARSER_ERROR: &str = r#"This is a bad string"#;

fn main() {
    let sql_result = get_string_without_error();
    println!("sql_result = {:?}\n", sql_result);

    /*
    let value_predicate = predicate::str::contains(
        r#"message: \"Error while planning query: SQL error: ParserError"#,
    );
    */

    let str = sql_result.unwrap_or("dog".to_string());

    let value_predicate = predicate::str::contains(str);

    let parse_error = value_predicate.eval(SQL_PARSER_ERROR);

    if parse_error {
        println!("got a parse error");
    } else {
        println!("did not get a parser error");
    }
}

pub fn get_string_without_error() -> Result<String, std::io::Error> {
    let x = "This is a good string".to_string();
    Ok(x)
}

pub fn get_string_with_error() -> Result<String, std::io::Error> {
    let x = "This is a bad string".to_string();
    Ok(x)
}
