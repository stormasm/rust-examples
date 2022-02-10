/*
I want a function that returns a bool true if no errors,
false if there in an error.
and inside this function is a match arm which tests conditions.
*/

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ValueError {
    details: String,
}

impl ValueError {
    fn new(msg: &str) -> ValueError {
        ValueError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ValueError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() {
    let v1 = Value::Int { val: 5 };
    let v2 = Value::Int { val: 2 };
    let v3 = Value::Int { val: 2 };
    let v4 = Value::Int { val: 4 };
    let v5 = Value::Bool { val: true };
    let v6 = Value::Int { val: 1 };
    let vec = vec![v1, v2, v3, v4, v5, v6];

    let values: Vec<_> = vec
        .windows(2)
        .map(|elem| process_check(&elem[0], &elem[1]))
        .collect();

    // Go back to the old nushell sort_by and see how they do it now
    // It looks like I don't have to call collect above but rather
    // when we find our first error blast out of it and return an error
    // otherwise for a long list you are going all the way to the end
    // when you don't really need to...

    for v in values {
        if v.is_err() {
            // In my real world scenario I will return an Err,
            // and if no error keep going and move on to vec.sort_by
            println!("{:?}", v);
        }
    }
}

// return true if there is no error
pub fn process_check(left: &Value, right: &Value) -> Result<bool, ValueError> {
    // println!("{:?} {:?}", left, right);

    let result = Ok(match (left, right) {
        (Value::Int { val: left, .. }, Value::Int { val: right, .. }) => {
            CompareValues::Ints(*left, *right).compare()
        }
        (Value::String { val: left, .. }, Value::String { val: right, .. }) => {
            CompareValues::String(left.clone(), right.clone()).compare()
        }

        // Ints will always come before strings
        (Value::Int { .. }, Value::String { .. }) => Some(Ordering::Less),
        (Value::String { .. }, Value::Int { .. }) => Some(Ordering::Greater),

        _ => {
            let x = format!("not able to compare {:?} with {:?}\n", left, right);
            // println!("left = {:?} right = {:?}\n", left, right);
            return Err(ValueError::new(&x));
        }
    });

    // println!("process_check result: {:?}\n", result);

    if result == Err(..) {
        return Ok(false);
    }
    return Ok(true);
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
