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

use crossbeam::crossbeam_channel::unbounded;

#[allow(dead_code)]
fn read_file_to_buffer1(filename: String) -> std::io::Result<()> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("bytes = {}\n {}", len, line);
    Ok(())
}

#[allow(dead_code)]
fn read_file_to_buffer2(filename: String) {
    // Create a channel of unbounded capacity.
    let (s, r) = unbounded();

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    //    let f = File::open(filename);
    //    let file = BufReader::new(f);

    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();
        // writeln!(writer, "{0} {1}\n", num, l).unwrap();

        let msg = r.recv().unwrap();
        println!("{}", msg);
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

    let _contents = read_file_to_buffer2(filename.to_string());

    // println!("With text:\n{:?}", contents);
}
