use std::collections::HashMap;

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
    let time_stamp1 = "1583712000".to_string();
    let time_stamp2 = "1583798400".to_string();
    let time_stamp3 = "1583884800".to_string();
    let field_set1 = get_fieldset("348000.00".to_string(), "127.21".to_string());
    let field_set2 = get_fieldset("245000.00".to_string(), "125.41".to_string());
    let field_set3 = get_fieldset("121000.00".to_string(), "134.21".to_string());

    let point1: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp1,
        fieldset: field_set1,
        tagset: get_tagset(),
    };

    let point2: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp2,
        fieldset: field_set2,
        tagset: get_tagset(),
    };

    let point3: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp3,
        fieldset: field_set3,
        tagset: get_tagset(),
    };

    let mut vec: Vec<Point> = Vec::new();

    vec.push(point1);
    vec.push(point2);
    vec.push(point3);

    println!("{:?}", vec);
}
