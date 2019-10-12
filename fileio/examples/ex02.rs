use std::error::Error;
use std::path::Path;
use std::{env, fs};

fn dir_reader() -> Result<(), Box<Error>> {
    let current_dir = Path::new("/tmp09/rust-hackernews/hn00/data/in");
    println!(
        "Entries in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        println!("{:?}", path.file_name().ok_or("No filename")?);

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}

fn main() {
    dir_reader();
}
