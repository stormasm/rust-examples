/*
use tempfile::tempfile;
use std::io::{self, Write};

use std::io::{Write, Result};

fn main() -> Result<()> {


// fn main() {
// Create a file inside of `std::env::temp_dir()`.
let mut file = tempfile();

writeln!(file, "Ralph was here. Then he left.")?;
}
*/

use std::io::{Result, Write};
use tempfile::tempfile;

fn test1() -> Result<()> {
    // Create a file inside of `std::env::temp_dir()`.
    let mut file = tempfile()?;
    writeln!(file, "Ralph was here. Then he left.")?;
    Ok(())
}

fn main() -> Result<()> {
    let _ = test1();

    /*
        let mut w = Vec::new();
        writeln!(&mut w)?;
        writeln!(&mut w, "test")?;
        writeln!(&mut w, "formatted {}", "arguments")?;

        assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());
    */
    Ok(())
}
