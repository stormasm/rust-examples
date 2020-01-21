
// https://doc.rust-lang.org/std/io/trait.Read.html

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("sample1.json")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    f.read(&mut buffer)?;
    println!("{:?}",buffer);

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    println!("{:?}",buffer);

    // read into a String, so that you don't need to do the conversion.
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    println!("{:?}",buffer);

    // and more! See the other methods for more details.
    Ok(())
}
