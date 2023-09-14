use predicates::prelude::*;

const SQL_PARSER_ERROR: &str = r#"This is a bad string"#;

fn main() {
    let sql_result = getStringWithError();
    println!("sql_result = {:?}\n", sql_result);

    /*
    let value_predicate = predicate::str::contains(
        r#"message: \"Error while planning query: SQL error: ParserError"#,
    );
    */

    let value_predicate = predicate::str::contains(sql_result);

    let parse_error = value_predicate.eval(SQL_PARSER_ERROR);

    if parse_error {
        println!("got a parse error");
    } else {
        println!("did not get a parser error");
    }
}

pub fn getStringWithoutError() -> Result<String, std::io::Error> {
    let x = "This is a good string".to_string();
    Ok(x)
}

pub fn getStringWithError() -> Result<String, std::io::Error> {
    let x = "This is a bad string".to_string();
    Ok(x)
}
