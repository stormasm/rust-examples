use std::env;
fn main() {
    println!("cwd = {:?}", env::current_dir().unwrap().display());
    println!("Hello, world!");
}
