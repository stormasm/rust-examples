use redis::Commands;
use redis::FromRedisValue;
// use serde::{Serialize, Serializer};
//use serde;
use serde_json::value::Value;

pub struct Job {
    pub class: String,
    pub args: Vec<Value>,
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: u64,
    pub enqueued_at: u64,
}

fn fetch_json() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut p1 = from_redis_value(con.lindex("development:queue:maman",0));

    let mut p2: Job = serde_json::from_str(p1).unwrap();
    println!("{:?}", p2);
    con.lindex("development:queue:maman",0)
}

fn fetch_a_string() -> redis::RedisResult<String> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.lindex("development:queue:maman",0)
}

fn main() {
    println!("Hello, redis!");
    let _y = fetch_a_string();
    println!("{:?}",_y);
    fetch_json();
}
