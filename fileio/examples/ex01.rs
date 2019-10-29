/*
*     For now this is the same as ex00
*     Eventually I will read file names from a directory...
*/

use std::env;
// use std::fs;
use std::process;
use std::string::String;

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn read_dir_to_vector(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a directory name");
        process::exit(1);
    }
    let dirname = &args[1];
    println!("In directory {}", dirname);

    let contents = read_dir_to_vector(dirname.to_string());

    println!("With text:\n{}", contents);
}
