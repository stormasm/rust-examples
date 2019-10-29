use std::env;
use std::process;
use std::string::String;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_file_to_buffer(filename: String) -> std::io::Result<()> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let _contents = read_file_to_buffer(filename.to_string());

    //println!("With text:\n{}", contents);
}
