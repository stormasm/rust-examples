use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    measurement: String,
    timestamp: String,
    fieldset: HashMap<String, String>,
    tagset: HashMap<String, String>,
}

fn get_fieldset1() -> HashMap<String, String> {
    let mut foo = HashMap::new();
    foo.insert("volume".to_string(), "344000.00".to_string());
    foo.insert("close".to_string(), "127.85".to_string());
    foo
}

fn get_fieldset2() -> HashMap<String, String> {
    let mut bar = HashMap::new();
    bar.insert("volume".to_string(), "144000.00".to_string());
    bar.insert("close".to_string(), "125.85".to_string());
    bar
}

fn get_fieldset3() -> HashMap<String, String> {
    let mut baz = HashMap::new();
    baz.insert("volume".to_string(), "244000.00".to_string());
    baz.insert("close".to_string(), "126.85".to_string());
    baz
}

fn get_ctagset() -> HashMap<String, String> {
    let mut foo = HashMap::new();

    foo.insert("frequency".to_string(), "daily".to_string());
    foo.insert("type".to_string(), "close".to_string());
    foo.clone()
}

fn main() {
    // Create struct with field init shorthand
    let _measurement = "ui".to_string();
    let time_stamp1 = "1583712000".to_string();
    let time_stamp2 = "1583798400".to_string();
    let time_stamp3 = "1583884800".to_string();
    let field_set1 = get_fieldset1();
    let field_set2 = get_fieldset2();
    let field_set3 = get_fieldset3();

    // Instantiate a `Point`
    let point1: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp1,
        fieldset: field_set1,
        tagset: get_ctagset(),
    };

    let point2: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp2,
        fieldset: field_set2,
        tagset: get_ctagset(),
    };

    let point3: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp3,
        fieldset: field_set3,
        tagset: get_ctagset(),
    };

    /*
        let point1 = Point {
            "ui".to_string(),
            time_stamp1,
            field_set1,
            get_ctagset(),
        };
        let point2 = Point {
            "ui".to_string(),
            time_stamp2,
            field_set2,
            get_ctagset(),
        };
        let point3 = Point {
            "ui".to_string(),
            time_stamp3,
            field_set3,
            get_ctagset(),
        };
    */
    // Print debug struct
    println!("{:?}", point1);
    println!("{:?}", point2);
    println!("{:?}", point3);
}
