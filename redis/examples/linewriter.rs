use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io::BufRead;

use serde_json::{Result, Value};
use redis::Commands;

fn write_json_to_redis_key(key: String, value: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    // let _: () = con.set(key, value)?;

    redis::cmd("SADD").arg("linejson").arg(value).execute(&mut con);

    Ok(())
}




/*
fn write_json_to_redis(json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    // you must convert &str to String
    // let vy = &vec[i].as_str().unwrap().to_string();

    let vy = json.as_str();

    redis::cmd("SADD").arg("linejson").arg(vy).execute(&mut con);

    Ok(())
}
*/

fn write_line_to_json(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {

        // let () = line.unwrap();
        let myline = line.unwrap();
        println!("{} {}\n", num, myline);

        write_json_to_redis_key("bingo".to_string(),myline);

        // let json = line.unwrap();
        // json1(json).expect("error converting json 1");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let _contents2 = write_line_to_json(filename.to_string());

    //println!("ok With text:\n{}", contents);
}
