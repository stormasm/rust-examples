fn main() {
    let v1 = Value::Int { val: 3 };
    let v2 = Value::Int { val: 2 };
    let v3 = Value::Int { val: 1 };
    let v4 = Value::Int { val: 4 };
    let v5 = Value::Int { val: 5 };
    let v6 = Value::Int { val: 7 };
    let v7 = Value::Int { val: 8 };
    let v8 = Value::Int { val: 10 };
    let v9 = Value::Int { val: 9 };

    let mut vec = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9];
    vec.sort_by(|a, b| process(a, b));
    println!("{:?}", vec);
}

pub fn process(left: &Value, right: &Value) -> std::cmp::Ordering {
    println!("{:?} {:?}", left, right);
    std::cmp::Ordering::Less
}

#[derive(Debug)]
pub enum Value {
    Bool { val: bool },
    Int { val: i64 },
    Float { val: f64 },
    String { val: String },
}
