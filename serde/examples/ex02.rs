extern crate serde;
extern crate serde_json;

// use serde_json::{Value};

use serde_json::{Result, Value};

fn t1() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

fn main() {
    println!("Hello, Bill!");

    let data = r#"
            {["abc","def","ghi"]
            }"#;

    //    let v: Value = serde_json::from_str(data);

    //    println!("{}", v[0]);

    t1();
}
