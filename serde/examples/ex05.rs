use serde_json::{Result, Value};

fn t1() -> Result<()> {
    let data = r#"
[
  {
   "a": "jim",
   "b": "susie"
  },
  {
   "a": 3,
   "b": 4
  }
]
"#;

    let v: Value = serde_json::from_str(data)?;
    println!("{:?}", v);

    Ok(())
}

fn main() {
    println!("Hello, Bill!");
    let _v = t1();
}
