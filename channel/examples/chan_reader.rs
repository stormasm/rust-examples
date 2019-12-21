use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use crossbeam::crossbeam_channel::unbounded;
use std::io::BufRead;

fn read_file_to_buffer(filename: String) {
    // Create a channel of unbounded capacity.
    let (s, r) = unbounded();

    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    for (_num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();

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

    let _contents = read_file_to_buffer(filename.to_string());

    // println!("With text:\n{:?}", contents);
}
