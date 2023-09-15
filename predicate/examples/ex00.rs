use predicates::prelude::*;

const STRING23: &str = "Two Three";

fn main() {
    let result = get_string_with_two_three();
    println!("result = {:?}\n", result);

    /*
    let value_predicate = predicate::str::contains(
        r#"message: \"Error while planning query: SQL error: ParserError"#,
    );
    */

    let mystr = result.unwrap_or("dog".to_string());
    println!("{:?}", mystr);

    let value_predicate = predicate::str::contains(mystr);

    let gothit = value_predicate.eval(STRING23);
    println!("parse_error = {:?}", gothit);

    if gothit {
        println!("got a string with a two three");
    } else {
        println!("got a string without two three");
    }
}

pub fn get_string_with_two_three() -> Result<String, std::io::Error> {
    let x = "Two Three Four".to_string();
    Ok(x)
}

pub fn get_string_without_two_three() -> Result<String, std::io::Error> {
    let x = "Four Five Six".to_string();
    Ok(x)
}
