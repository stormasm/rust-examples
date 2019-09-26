extern crate redis;
use redis::Commands;

fn fetch_a_string() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.lindex("development:queue:maman",1)
}

fn main() {
    println!("Hello, redis!");
    let _y = fetch_a_string();
    println!("{:?}",_y)
}
