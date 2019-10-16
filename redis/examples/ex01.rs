extern crate redis;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key120", 120)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    //

    let _: () = con.hset("hm-120", "58", "hola mike")?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.

    // let x1 : redis::RedisResult = con.hget("hm-120","120");

    con.get("my_key120")
}

fn main() {
    println!("Hello, redis!");
    let _y = fetch_an_integer();
    println!("{:?}", _y)
}
