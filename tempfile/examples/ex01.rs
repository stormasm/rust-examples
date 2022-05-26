// alias tdir='export TMPDIR="/Users/ma/j/tmp33"'

use std::env;

fn main() {
    let dir = env::temp_dir();
    println!("Temporary directory: {}", dir.display());
}
