extern crate redis;
use redis::Commands;
//use std::error::Error;

fn do_something() -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    let _: () = con.set("my_key124", 124)?;
    con.get("my_key124")
}

fn main() {
    match do_something() {
        Err(err) => {
            println!("Could not execute example:");
            println!("{}", err.to_string());
        }
        Ok(value) => {
            println!("{}", value);
        }
    }
}
