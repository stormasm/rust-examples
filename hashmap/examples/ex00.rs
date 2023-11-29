/*
* This code started out in notrace/main.rs
*/

use std::collections::HashMap;

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io::BufRead;

fn read_file_to_buffer(mut map: HashMap<String, i32>, filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let l = line.unwrap();

        match map.get(&l) {
            Some(count) => {
                map.insert(l.to_string(), count + 1);
            }
            None => {
                map.insert(l.to_string(), 1);
            }
        }
    }
    print(map)
}

fn print(map: HashMap<String, i32>) {
    println!("Occurences..");

    for (key, value) in map.iter() {
        println!("{key}: {value}");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    let map = HashMap::new();
    let _contents = read_file_to_buffer(map, filename.to_string());
}
