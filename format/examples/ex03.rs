use std::collections::HashMap;

use std::fmt::Write as FmtWrite;
//use std::io::Write as IoWrite;

#[derive(Debug)]
struct Point {
    measurement: String,
    timestamp: String,
    fieldset: HashMap<String, String>,
    tagset: HashMap<String, String>,
}

impl Point {
    fn set_fieldset(volume: String, close: String) -> HashMap<String, String> {
        let mut foo = HashMap::new();
        foo.insert("volume".to_string(), volume);
        foo.insert("close".to_string(), close);
        foo.clone()
    }

    fn set_tagset() -> HashMap<String, String> {
        let mut foo = HashMap::new();
        foo.insert("frequency".to_string(), "daily".to_string());
        foo.insert("type".to_string(), "close".to_string());
        foo.clone()
    }

    fn get_tagset(self) -> Result<String, Box<dyn std::error::Error>> {
        //    fn get_fieldset(self) -> String {
        let mut s = String::new();
        for (key, val) in self.tagset {
            write!(&mut s, "{}={},", key, val).expect("ok1");
        }
        let strlen = s.len();
        let mut s1 = String::from(s);
        s1.remove(strlen - 1);
        println!("{}", s1);
        Ok(s1)
    }
}
fn main() {
    let point: Point = Point {
        measurement: "ui".to_string(),
        timestamp: "1583712000".to_string(),
        fieldset: Point::set_fieldset("348000.00".to_string(), "127.21".to_string()),
        tagset: Point::set_tagset(),
    };
    let x = point.get_tagset().unwrap();
    println!("{:?}", x);
}
