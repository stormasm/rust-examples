use std::collections::HashSet;

/*
*  Check to see if any of the columns inside the input
*  does not exist in a vec of columns
*/

pub fn column_does_not_exist(inputs: Vec<String>, columns: Vec<String>) -> bool {
    let mut set = HashSet::new();
    for column in columns {
        set.insert(column);
    }

    for input in &inputs {
        if set.contains(input) {
            continue;
        }
        return true;
    }
    return false;
}

/*
*  Check to see if a column exists in a vec of columns
*/

pub fn column_exists(input: Vec<String>, columns: Vec<String>) -> bool {
    for column in columns {
        if let Some(_index) = input.iter().position(|value| *value == column) {
            return true;
        }
    }
    false
}

fn main() {
    let input1 = vec!["black".to_string(), "pink".to_string()];
    let columns1 = vec!["green".to_string(), "blue".to_string()];
    let result = column_exists(input1, columns1);
    println!("Exists = {:?}", result);

    let input2 = vec!["red".to_string(), "green".to_string()];
    let columns2 = vec!["green".to_string(), "blue".to_string()];
    let result = column_does_not_exist(input2, columns2);
    println!("Does not exist = {:?}", result);
}
