/*
*  Check to see if a column does not exist in a vec of columns
*/

fn column_does_not_exist(input: Vec<String>, columns: Vec<String>) -> bool {
    for column in columns {
        if let Some(_index) = input.iter().position(|value| *value == column) {
            return false;
        }
    }
    true
}

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
    let input1 = vec!["red".to_string(), "white".to_string()];
    let columns1 = vec!["green".to_string(), "blue".to_string()];
    let result = column_exists(input1, columns1);
    println!("Exists = {:?}", result);

    let input2 = vec!["red".to_string(), "white".to_string()];
    let columns2 = vec!["green".to_string(), "blue".to_string()];
    let result = column_does_not_exist(input2, columns2);
    println!("Does not exist = {:?}", result);
}
