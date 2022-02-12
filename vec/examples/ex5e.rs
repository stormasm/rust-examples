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
    // to test uncomment this v4
    // and comment the next v4
    //let v4 = Value::Bool { val: true };
    let v4 = Value::Int { val: 4 };
    let v5 = Value::String {
        val: "a".to_string(),
    };
    let v6 = Value::Int { val: 1 };
    let v7 = Value::Int { val: 8 };
    let mut vec = vec![v1, v2, v3, v4, v5, v6, v7];

    if let Some(values) = vec
        .windows(2)
        .map(|elem| process_check(&elem[0], &elem[1]))
        .find(|elem| elem.is_err())
    {
        println!("{:?}", values);
    }
    vec.sort_by(|a, b| process_check(a, b).expect("sort_by default bug"));
    println!("{:?}", vec);
}

pub fn process_check(left: &Value, right: &Value) -> Result<Ordering, ValueError> {
    //println!("{:?} {:?}", left, right);

    Ok(match (left, right) {
        (Value::Int { val: left, .. }, Value::Int { val: right, .. }) => {
            CompareValues::Ints(*left, *right).compare()
        }
        (Value::String { val: left, .. }, Value::String { val: right, .. }) => {
            CompareValues::String(left.clone(), right.clone()).compare()
        }

        // Ints will always come before strings
        (Value::Int { .. }, Value::String { .. }) => Ordering::Less,
        (Value::String { .. }, Value::Int { .. }) => Ordering::Greater,

        _ => {
            let x = format!("not able to compare {:?} with {:?}\n", left, right);
            // println!("left = {:?} right = {:?}\n", left, right);
            return Err(ValueError::new(&x));
        }
    })
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
}

impl CompareValues {
    pub fn compare(&self) -> std::cmp::Ordering {
        match self {
            CompareValues::Ints(left, right) => left.cmp(right),
            CompareValues::String(left, right) => left.cmp(right),
        }
    }
}
