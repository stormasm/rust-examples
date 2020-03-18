use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    println!("Entries in {:?}:", mypath);

    let mut vec: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        //println!("{:?}", path.file_name().ok_or("No filename")?);
        vec.push(path);
    }

    Ok(vec)
}

#[allow(dead_code)]
fn file_stem(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    let name = path.file_stem().unwrap().to_str();
    println!("{:?}", name);
    name
}

fn processor(mydir: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(mydir).unwrap();
    // println!("vec len = {:?}", vec.len());

    for name in vec {
        // let filename = name.file_name().ok_or("No filename")?;
        let filename = name.file_name();
        let filestem = name.file_stem();
        println!("{:?} {:?}", filename, filestem);
        // println!("{:?}", file_stem(filename));
    }

    Ok(())
}

fn main() {
    let mydir = String::from("/tmp13");
    let _ = processor(mydir);
}
