use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use crossbeam::crossbeam_channel::{unbounded, Receiver};
use std::io::BufRead;

fn process_lines(r: Receiver<String>) {
    let msg = r.recv().unwrap();
    println!("{}", msg);
}

fn read_file_to_buffer(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    for (_num, line) in file.lines().enumerate() {
        // Create a channel of unbounded capacity.
        let (s, r) = unbounded();

        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();
        process_lines(r);
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

    let _contents = read_file_to_buffer(filename.to_string());

    // println!("With text:\n{:?}", contents);
}
