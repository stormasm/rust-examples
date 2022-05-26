use std::io::{Result, Write};
use tempfile::tempfile;

fn test1() -> Result<()> {
    // Create a file inside of `std::env::temp_dir()`.
    let mut file = tempfile()?;
    println!("{:?}", file);
    writeln!(file, "Ralph was here. Then he left.")?;
    Ok(())
}

fn main() -> Result<()> {
    let _ = test1();
    Ok(())
}
