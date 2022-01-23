/*
*  Check to see if a column exists in a vec of columns
*/

fn column_exists(input: Vec<String>, columns: Vec<String>) -> bool {
    for column in columns {
        if let Some(_index) = input.iter().position(|value| *value == column) {
            return true;
        }
    }
    false
}

fn main() {
    let input = vec!["red".to_string(), "white".to_string()];
    let columns = vec!["green".to_string(), "blue".to_string()];
    let result = column_exists(input, columns);
    println!("{:?}", result);
}
