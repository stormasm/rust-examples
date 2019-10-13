use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn dir_reader() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // fn dir_reader() -> Result<Vec<&str>,_>  {
    let current_dir = Path::new("/tmp09/rust-hackernews/hn00/data/in");
    println!("Entries in {:?}:", current_dir);

    let mut vec: Vec<PathBuf> = Vec::new();

    // let mut vec: Vec<&str> = vec![&str];

    //    let vec: Vec<str> = Vec::new();
    //    vec.resize(len, 0);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        println!("{:?}", path.file_name().ok_or("No filename")?);

        vec.push(path);
    }

    Ok(vec)
}

fn processor() -> Result<(), Box<dyn Error>> {
    //fn processor() -> Result(_,_) {
    let vec = dir_reader().unwrap();

    println!("vec len = {:?}", vec.len());

    for name in vec {
        // println!("{:?}", name.path.file_name().ok_or("No filename")?);
        // println!("{:?}", name.capacity());

        // let () = name;

        println!("{:?}", name.file_name().ok_or("No filename")?);

        //println!("{:?}", name.file_name);
    }

    Ok(())
}

fn main() {
    let _ = processor();
}
