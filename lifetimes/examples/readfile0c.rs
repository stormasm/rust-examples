// https://doc.rust-lang.org/reference/types/tuple.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use std::convert::TryInto;

fn is_even(num: u32) -> bool {
    (num) & 1 == 0
}

fn readfile(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    // let mut myline = String::new();
    //
    let mut mynum: i32;
    let mut myline: String;

    let mut vec_key = Vec::new();
    let mut vec_value = Vec::new();

    let mut writer = BufWriter::new(io::stdout());
    for (mynum, myline) in file.lines().enumerate() {
        //mynum = &num;
        //myline = &line.unwrap().clone();
        if is_even(mynum.try_into().unwrap()) {
            writeln!(writer, "{0}\n", mynum).unwrap();
            //writeln!(writer, "{0}\n", myline).unwrap();
            vec_key.push(&mynum);
        }
        if !is_even(mynum.try_into().unwrap()) {
            writeln!(writer, "{0}\n", mynum).unwrap();
            //writeln!(writer, "{0}\n", myline).unwrap();
            vec_value.push(&myline);
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
    println!("In file {}", filename);

    let _contents = readfile(filename.to_string());
}
