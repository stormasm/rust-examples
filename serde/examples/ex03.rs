use serde_json::{Result, Value};

fn t1() -> Result<()> {
    let data = r#"
            [
                "abc",
                "def",
                "ghi"
            ]
        "#;

    let v: Value = serde_json::from_str(data)?;

    // Convert the serde value to a Vector
    let vec = v.as_array().unwrap();

    // iterate over the vector
    for i in 0..vec.len() {
        let vx = &vec[i].as_str().unwrap();
        println!("{}", vx);
    }

    Ok(())
}

fn main() {
    println!("Hello, Bill!");
    let _v = t1();
}
