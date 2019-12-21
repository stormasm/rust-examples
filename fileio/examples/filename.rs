use std::fs;
use std::string::String;

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn main() {
    let filename = String::from("./ex00.txt");
    println!("In file {}", filename);

    let contents = read_file_to_string(filename.to_string());

    println!("With text:\n{}", contents);
}
