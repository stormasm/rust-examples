/*
Arbitrary Order of Values:
    Floats
    Ints
    Strings
    Bools
*/

use std::cmp::Ordering;

fn main() {
    let s1 = Span { start: 0, end: 1 };

    let v1 = Value::Int { val: 3, span: s1 };
    let v2 = Value::Float { val: 2.1, span: s1 };
    let v3 = Value::Bool {
        val: true,
        span: s1,
    };
    let v4 = Value::Int { val: 4, span: s1 };
    let v5 = Value::String {
        val: "x".to_string(),
        span: s1,
    };
    let v6 = Value::Bool {
        val: false,
        span: s1,
    };
    let v7 = Value::Int { val: 8, span: s1 };

    let v8 = Value::String {
        val: "a".to_string(),
        span: s1,
    };

    let v9 = Value::Filesize { val: 41, span: s1 };
    let v10 = Value::Float { val: 9.2, span: s1 };
    let v11 = Value::Float { val: 2.1, span: s1 };
    let v12 = Value::Int { val: 8, span: s1 };

    let mut vec = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12];

    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    let values: Vec<_> = vec
        .windows(2)
        .map(|elem| process_check(&elem[0], &elem[1]))
        .collect();
    println!("values: {:?}", values);

    if values.contains(&false) {
        println!("fail")
    } else {
        println!("pass");
        vec.sort_by(|a, b| process(a, b));
        println!("{:?}", vec);
    }
}

// return true if there is no error
pub fn process_check(left: &Value, right: &Value) -> bool {
    println!("{:?} {:?}", left, right);

    let result = match (left, right) {
        (Value::Float { val: left, .. }, Value::Float { val: right, .. }) => {
            CompareValues::Floats(*left, *right).compare_check()
        }
        (Value::Int { val: left, .. }, Value::Int { val: right, .. }) => {
            CompareValues::Ints(*left, *right).compare_check()
        }
        (Value::String { val: left, .. }, Value::String { val: right, .. }) => {
            CompareValues::String(left.clone(), right.clone()).compare_check()
        }
        (Value::Bool { val: left, .. }, Value::Bool { val: right, .. }) => {
            CompareValues::Booleans(*left, *right).compare_check()
        }

        // Floats will always come before Ints
        (Value::Float { .. }, Value::Int { .. }) => Some(Ordering::Less),
        (Value::Int { .. }, Value::Float { .. }) => Some(Ordering::Greater),

        // Floats will always come before Strings
        (Value::Float { .. }, Value::String { .. }) => Some(Ordering::Less),
        (Value::String { .. }, Value::Float { .. }) => Some(Ordering::Greater),

        // Floats will always come before Bools
        (Value::Float { .. }, Value::Bool { .. }) => Some(Ordering::Less),
        (Value::Bool { .. }, Value::Float { .. }) => Some(Ordering::Greater),

        // Ints will always come before strings
        (Value::Int { .. }, Value::String { .. }) => Some(Ordering::Less),
        (Value::String { .. }, Value::Int { .. }) => Some(Ordering::Greater),

        // Ints will always come before Bools
        (Value::Int { .. }, Value::Bool { .. }) => Some(Ordering::Less),
        (Value::Bool { .. }, Value::Int { .. }) => Some(Ordering::Greater),

        // Strings will always come before Bools
        (Value::String { .. }, Value::Bool { .. }) => Some(Ordering::Less),
        (Value::Bool { .. }, Value::String { .. }) => Some(Ordering::Greater),

        _ => None,
    };
    println!("process_check result: {:?}\n", result);

    if result == None {
        return false;
    }
    true
}

pub fn process(left: &Value, right: &Value) -> std::cmp::Ordering {
    //println!("{:?} {:?}", left, right);

    match (left, right) {
        (Value::Float { val: left, .. }, Value::Float { val: right, .. }) => {
            CompareValues::Floats(*left, *right).compare()
        }
        (Value::Int { val: left, .. }, Value::Int { val: right, .. }) => {
            CompareValues::Ints(*left, *right).compare()
        }
        (Value::String { val: left, .. }, Value::String { val: right, .. }) => {
            CompareValues::String(left.clone(), right.clone()).compare()
        }
        (Value::Bool { val: left, .. }, Value::Bool { val: right, .. }) => {
            CompareValues::Booleans(*left, *right).compare()
        }

        // Floats will always come before Ints
        (Value::Float { .. }, Value::Int { .. }) => Ordering::Less,
        (Value::Int { .. }, Value::Float { .. }) => Ordering::Greater,

        // Floats will always come before Strings
        (Value::Float { .. }, Value::String { .. }) => Ordering::Less,
        (Value::String { .. }, Value::Float { .. }) => Ordering::Greater,

        // Floats will always come before Bools
        (Value::Float { .. }, Value::Bool { .. }) => Ordering::Less,
        (Value::Bool { .. }, Value::Float { .. }) => Ordering::Greater,

        // Ints will always come before strings
        (Value::Int { .. }, Value::String { .. }) => Ordering::Less,
        (Value::String { .. }, Value::Int { .. }) => Ordering::Greater,

        // Ints will always come before Bools
        (Value::Int { .. }, Value::Bool { .. }) => Ordering::Less,
        (Value::Bool { .. }, Value::Int { .. }) => Ordering::Greater,

        // Strings will always come before Bools
        (Value::String { .. }, Value::Bool { .. }) => Ordering::Less,
        (Value::Bool { .. }, Value::String { .. }) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

#[derive(Debug)]
pub enum Value {
    Bool { val: bool, span: Span },
    Int { val: i64, span: Span },
    Float { val: f64, span: Span },
    String { val: String, span: Span },
    Filesize { val: i64, span: Span },
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum CompareValues {
    Ints(i64, i64),
    Floats(f64, f64),
    String(String, String),
    Booleans(bool, bool),
    None,
}

impl CompareValues {
    pub fn compare_check(&self) -> Option<std::cmp::Ordering> {
        match self {
            CompareValues::Ints(left, right) => Some(left.cmp(right)),
            CompareValues::Floats(left, right) => Some(process_floats(left, right)),
            CompareValues::String(left, right) => Some(left.cmp(right)),
            CompareValues::Booleans(left, right) => Some(left.cmp(right)),
            _ => None,
        }
    }
    pub fn compare(&self) -> std::cmp::Ordering {
        match self {
            CompareValues::Ints(left, right) => left.cmp(right),
            CompareValues::Floats(left, right) => process_floats(left, right),
            CompareValues::String(left, right) => left.cmp(right),
            CompareValues::Booleans(left, right) => left.cmp(right),
            _ => Ordering::Equal,
        }
    }
}

pub fn process_floats(left: &f64, right: &f64) -> std::cmp::Ordering {
    let result = left.partial_cmp(right);
    match result {
        Some(Ordering::Greater) => Ordering::Greater,
        Some(Ordering::Less) => Ordering::Less,
        _ => Ordering::Equal,
    }
}
