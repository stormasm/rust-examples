/*
* This code started out in fileio/examples/linereader.rs
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

fn check_log_statement(line: String) -> bool {
    line.contains("trace")
}

// Remove the ! operator to see all of the log statements for debugging

fn read_file_to_buffer(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut writer = BufWriter::new(io::stdout());
    for (_num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if !check_log_statement(l.clone()) {
            writeln!(writer, "{}", l).unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    let _contents = read_file_to_buffer(filename.to_string());
}
