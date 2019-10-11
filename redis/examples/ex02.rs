extern crate redis;
use redis::{Commands};
use std::error::Error;

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _ : () = con.set("my_key124", 124)?;
    con.get("my_key121")
}

fn main() {
    match do_something() {
        Err(err) => {
            println!("Could not execute example:");
            println!("  {}: {}", err.category(), err.description());
        }
        Ok(()) => {}
    }
}
