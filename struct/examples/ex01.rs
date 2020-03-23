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

fn get_tagset1() -> HashMap<String, String> {
    let mut foo = HashMap::new();

    foo.insert("frequency".to_string(), "daily".to_string());
    foo.insert("type".to_string(), "close".to_string());
    foo
}

fn get_tagset2() -> HashMap<String, String> {
    let mut foo = HashMap::new();

    foo.insert("frequency".to_string(), "daily".to_string());
    foo.insert("type".to_string(), "close".to_string());
    foo
}
fn get_tagset3() -> HashMap<String, String> {
    let mut foo = HashMap::new();

    foo.insert("frequency".to_string(), "daily".to_string());
    foo.insert("type".to_string(), "close".to_string());
    foo
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
    //let tag_set = get_tagset();

    // Instantiate a `Point`
    let point1: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp1,
        fieldset: field_set1,
        tagset: get_tagset1(),
    };

    let point2: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp2,
        fieldset: field_set2,
        tagset: get_tagset2(),
    };

    let point3: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp3,
        fieldset: field_set3,
        tagset: get_tagset3(),
    };

/*
    let point1 = Point {
        measurement,
        timestamp1,
        field_set1,
        tag_set,
    };
    let point2 = Point {
        measurement,
        timestamp2,
        field_set2,
        tag_set,
    };
    let point3 = Point {
        measurement,
        timestamp3,
        field_set3,
        tag_set,
    };
*/
    // Print debug struct
    println!("{:?}", point1);
    println!("{:?}", point2);
    println!("{:?}", point3);

/*
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    */
}
