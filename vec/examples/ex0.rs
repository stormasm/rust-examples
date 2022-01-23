/*
*  Check to see if any of the columns inside the input
*  does not exist in a vec of columns
*/

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn column_does_not_exist(inputs: Vec<String>, columns: Vec<String>) -> bool {
    if do_vecs_match(&inputs, &columns) {
        return false;
    }

    for column in columns {
        println!("column = {:?}", column);
        for input in &inputs {
            println!("input = {:?}", input);
            if *input != column {
                return true;
            } else {
                break;
            }
        }
    }
    false
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
    let input1 = vec!["blue".to_string(), "yellow".to_string()];
    let columns1 = vec!["green".to_string(), "blue".to_string()];
    let result = column_exists(input1, columns1);
    println!("Exists = {:?}", result);

    let input2 = vec!["orange".to_string(), "white".to_string()];
    let columns2 = vec!["green".to_string(), "blue".to_string()];
    let result = column_does_not_exist(input2, columns2);
    println!("Does not exist = {:?}", result);
}
