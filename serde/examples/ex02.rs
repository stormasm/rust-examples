use serde_json::{Result, Value};

fn t1() -> Result<()> {
    let data = r#"
            [
                "abc",
                "def"
            ]
        "#;

    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("with quotes {}", v[0]);

    let v1 = &v[0].as_str().unwrap();
    println!("without quotes = {}", v1);

    Ok(())
}

fn main() {
    println!("Hello, Bill!");
    let _v = t1();
}
