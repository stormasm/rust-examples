/*
* Ref: https://users.rust-lang.org/t/read-a-file-line-by-line/1585/7
*
*/

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use serde_json::{Result, Value};

fn json1(data: String) -> Result<()> {
    /*
        let data = r#"
                [
                    "abc",
                    "def",
                    "ghi"
                ]
            "#;
    */
    let v: Value = serde_json::from_str(&data)?;
    println!("{}\n", v);
    Ok(())
}

fn write_file_to_json(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    //    let f = File::open(filename);
    //    let file = BufReader::new(f);

    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        let json = line.unwrap();
        //writeln!(writer, "{0} {1}\n", num, json).unwrap();
        json1(json);
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

    let _contents = write_file_to_json(filename.to_string());

    //println!("With text:\n{}", contents);
}
