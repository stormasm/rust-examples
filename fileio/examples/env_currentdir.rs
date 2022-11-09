use std::env;

// pub fn current_dir() -> Result<PathBuf>
// Returns the current working directory as a PathBuf.
// https://doc.rust-lang.org/std/env/fn.current_dir.html

fn main() {
    let dir = env::current_dir().unwrap();
    println!("{:?}", dir);
}

// For more examples of how this is used please see
// datafusion/examples/ex00.rs
