use std::io::Write;
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

    let mut json1 = String::from(data);
    //println!("{:?}", json1);
    json1.retain(|c| !c.is_whitespace());
    // println!("{:?}", json1);

    let stdout = std::io::stdout();
    match stdout.lock().write_all(json1.as_bytes()) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    };

    let _v: Value = serde_json::from_str(&json1)?;
    //println!("{:?}", v);

    Ok(())
}

fn main() {
    println!("Hello, Bill!");
    let _v = t1();
}
