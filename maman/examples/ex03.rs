#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate serde;
extern crate serde_json;
use redis::Commands;

use serde_json::value::Value;

use std::collections::BTreeMap;
use url::{Url};

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    pub class: String,
    pub args: Vec<Value>,
    pub retry: i64,
    pub queue: String,
    pub jid: String,
    pub created_at: u64,
    pub enqueued_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub url: Url,
    pub document: String,
    pub headers: BTreeMap<String, String>,
    pub status: String,
    pub urls: Vec<Url>,
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let k: Option<String> = con.lindex("development:queue:maman", 0)?;
    let k1 = k.unwrap();

    let k2: Job = serde_json::from_str(&k1).unwrap();
    println!("Deserialized: {:?}", k2);

    let args = k2.args;
    for arg in args.iter() {
        let k3: Page = serde_json::from_str(arg).unwrap();
        println!("MyPage {:?}", k3)
    }

    Ok(())
}

fn main() {
    println!("Hello, redis!");
    do_something();
}
