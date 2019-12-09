// references
// https://www.reddit.com/r/rust/comments/8z4tez/how_to_convert_vec_to_character_in_rust/

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

fn readfilea() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("linesmin.txt")?;
    let mut file_copy = file.try_clone()?;

    file.seek(SeekFrom::Start(3))?;

    let mut contents = vec![];
    file_copy.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() {
    let contents = readfilea().unwrap();
    println!("{:?}", contents);
}
