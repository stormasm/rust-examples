use serde_json::{Result, Value};

fn t1() -> Result<()> {
    let data = r#"
            [200,300,400]
        "#;

    let v: Value = serde_json::from_str(data)?;

    // Convert the serde value to a Vector
    let vec = v.as_array().unwrap();
    println!("{:?}", vec);
    // iterate over the vector

    // let mut items = Vec::new();

    for i in 0..vec.len() {
        let vx = &vec[i].as_u64().unwrap();
        println!("{:?}", vx);
        // items.push(vx);
    }

    Ok(())
}

fn main() {
    println!("Hello, Bill!");
    let _v = t1();
}
