// https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines

#![allow(unused)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("file-to-vec.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
