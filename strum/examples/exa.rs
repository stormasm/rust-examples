// https://stackoverflow.com/questions/21371534/in-rust-is-there-a-way-to-iterate-through-the-values-of-an-enum
// https://docs.rs/strum_macros/latest/strum_macros/derive.EnumIter.html

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    for direction in Direction::iter() {
        println!("{:?}", direction);
    }
}
