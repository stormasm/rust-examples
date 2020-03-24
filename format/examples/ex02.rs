use std::collections::HashMap;
//use std::fmt::{self, Display, Formatter};
//use std::fmt::{self, Debug, Formatter};
use std::fmt::{self, Formatter};

// #[derive(Debug)]
struct Point {
    measurement: String,
    timestamp: String,
    fieldset: HashMap<String, String>,
    tagset: HashMap<String, String>,
}

impl fmt::Debug for Point {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        /*
                write!(f, "{}{} {}{}",
                       self.measurement, self.tagset, self.fieldset, self.timestamp)
        */
        write!(f, "{} {}", self.measurement, self.timestamp)
    }
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
