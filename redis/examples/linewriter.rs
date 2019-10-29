use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io::BufRead;

use serde_json::{Result, Value};

fn json1(data: String) -> Result<()> {
    let v: Value = serde_json::from_str(&data)?;
    println!("{}\n", v);
    Ok(())
}

fn write_line_to_json(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let json = line.unwrap();
        json1(json).expect("error converting json 1");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let _contents2 = write_line_to_json(filename.to_string());

    //println!("ok With text:\n{}", contents);
}
