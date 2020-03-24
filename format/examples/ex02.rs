use std::fmt::{self, Formatter};

struct Point {
    measurement: String,
    timestamp: String,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.measurement, self.timestamp)
    }
}

fn main() {
    let _measurement = "ui".to_string();
    let time_stamp = "1583712000".to_string();

    let point: Point = Point {
        measurement: "ui".to_string(),
        timestamp: time_stamp,
    };

    println!("{:?}", point);
}
