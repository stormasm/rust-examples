fn main() {
    let v1 = Value::Int { val: 4 };
    let v2 = Value::Int { val: 40 };
    let v3 = Value::Int { val: 10 };
    let v4 = Value::Int { val: 1 };

    let mut vec = vec![v1, v2, v3, v4];
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
