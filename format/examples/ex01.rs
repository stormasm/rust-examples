use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.measurement, self.timestamp)
    }
}

#[derive(Debug)]
struct Point {
    measurement: String,
    timestamp: String,
    fieldset: HashMap<String, String>,
    tagset: HashMap<String, String>,
}

fn get_fieldset(volume: String, close: String) -> HashMap<String, String> {
    let mut foo = HashMap::new();
    foo.insert("volume".to_string(), volume);
    foo.insert("close".to_string(), close);
    foo.clone()
}

fn get_tagset() -> HashMap<String, String> {
    let mut foo = HashMap::new();
    foo.insert("frequency".to_string(), "daily".to_string());
    foo.insert("type".to_string(), "close".to_string());
    foo.clone()
}

fn main() {
    let _measurement = "ui".to_string();
    let time_stamp = "1583712000".to_string();
    let field_set = get_fieldset("348000.00".to_string(), "127.21".to_string());

    let point: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp,
        fieldset: field_set,
        tagset: get_tagset(),
    };

    println!("{:?}", point);
}
