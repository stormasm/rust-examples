use infcsv::point::Point;

fn main() {
    let point: Point = Point {
        measurement: "ui".to_string(),
        tagset: Point::set_tagset(),
        fieldset: Point::set_fieldset("348000.00".to_string(), "127.21".to_string()),
        timestamp: "1583712000".to_string(),
    };
    let x = point.get_lineprotocol().unwrap();
    println!("{}", x);
}
