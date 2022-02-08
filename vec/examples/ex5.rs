/*
I want a function that returns a bool true if no errors,
false if there in an error.
and inside this function is a match arm which tests conditions.
*/

use std::cmp::Ordering;

fn main() {
    let v1 = Value::Int { val: 5 };
    let v2 = Value::Int { val: 2 };
    let v3 = Value::Int { val: 2 };
    let v4 = Value::Int { val: 4 };
    let v5 = Value::Bool { val: true };
    let vec = vec![v1, v2, v3, v4, v5];

    let values: Vec<_> = vec
        .windows(2)
        .map(|elem| process_check(&elem[0], &elem[1]))
        .collect();

    println!("values: {:?}", values);
}

// return true if there is no error
pub fn process_check(left: &Value, right: &Value) -> bool {
    println!("{:?} {:?}", left, right);

    let result = match (left, right) {
        (Value::Int { val: left, .. }, Value::Int { val: right, .. }) => {
            CompareValues::Ints(*left, *right).compare()
        }
        (Value::String { val: left, .. }, Value::String { val: right, .. }) => {
            CompareValues::String(left.clone(), right.clone()).compare()
        }

        // Ints will always come before strings
        (Value::Int { .. }, Value::String { .. }) => Some(Ordering::Less),
        (Value::String { .. }, Value::Int { .. }) => Some(Ordering::Greater),

        _ => None,
    };

    println!("process_check result: {:?}\n", result);
    true
}

#[derive(Debug)]
pub enum Value {
    Int { val: i64 },
    String { val: String },
    Bool { val: bool },
}

#[derive(Debug)]
pub enum CompareValues {
    Ints(i64, i64),
    String(String, String),
    None,
}

impl CompareValues {
    pub fn compare(&self) -> Option<std::cmp::Ordering> {
        match self {
            CompareValues::Ints(left, right) => Some(left.cmp(right)),
            _ => None,
        }
    }
}
