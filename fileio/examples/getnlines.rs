/*
* Ref: https://users.rust-lang.org/t/read-a-file-line-by-line/1585/7
*
*/

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io::BufRead;
use std::io::LineWriter;
use std::io::Write;

#[allow(dead_code)]
fn read_file_to_buffer1(filename: String) -> std::io::Result<()> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {} bytes long", len);
    Ok(())
}

fn read_file_to_buffer2(filename: String, numoflines: usize) -> std::io::Result<()> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let filew = File::create("tmp.txt")?;
    let mut filew = LineWriter::new(filew);

    filew.write_all(b"I shall be telling this with a sigh")?;

    for (num, line) in file.lines().enumerate() {
        if num < numoflines {
            let l = line.unwrap();
            writeln!(filew, "{0} {1}\n", num, l).unwrap();
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("You need to enter a filename and the number of lines to write out...");
        process::exit(1);
    }
    let filename = &args[1];
    let numoflines = &args[2];

    let nol = numoflines.parse::<usize>().unwrap();

    println!("Reading {} lines of file {}", nol, filename);

    let _contents = read_file_to_buffer2(filename.to_string(), nol);

    //println!("With text:\n{}", contents);
}
