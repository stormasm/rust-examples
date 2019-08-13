use std::env;
use std::fs;
use std::process;
use std::string::String;

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let contents = read_file_to_string(filename.to_string());

    println!("With text:\n{}", contents);
}
