use std::collections::HashMap;

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

    fn get_fieldset(self) -> String {
        for (key, val) in self.tagset {
            print!("{}={},", key, val);
        }
        "ok".to_string()
    }
}
fn main() {
    let point: Point = Point {
        measurement: "ui".to_string(),
        timestamp: "1583712000".to_string(),
        fieldset: Point::set_fieldset("348000.00".to_string(), "127.21".to_string()),
        tagset: Point::set_tagset(),
    };

    // println!("{:?}", point);
    println!("{:?}", point.get_fieldset());
}
