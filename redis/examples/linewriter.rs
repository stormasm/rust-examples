use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use std::string::String;

fn write_json_to_redis_set(key: String, value: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    let my = String::from("set-");
    let mykey = [my, key].concat();
    redis::cmd("SADD").arg(mykey).arg(value).execute(&mut con);
    Ok(())
}

#[allow(dead_code)]
fn write_json_to_redis_list(key: String, value: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    let my = String::from("list-");
    let mykey = [my, key].concat();
    redis::cmd("RPUSH").arg(mykey).arg(value).execute(&mut con);
    Ok(())
}

#[allow(dead_code)]
fn write_json_to_redis_hash(key: String, field: String, value: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    let my = String::from("hash-");
    let mykey = [my, key].concat();
    redis::cmd("HSET")
        .arg(mykey)
        .arg(field)
        .arg(value)
        .execute(&mut con);
    Ok(())
}

fn write_line_to_json(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().enumerate() {
        let myline = line.unwrap();
        // println!("{} {}\n", num, myline);
        // only one of the below lines ever get used
        // but leave here for reference on how to use {set, list, hash}
        let _k1 = write_json_to_redis_set("linejson".to_string(), myline);
        //let _k1 = write_json_to_redis_list("linejson".to_string(), myline);
        //let _k1 = write_json_to_redis_hash("linejson".to_string(), num.to_string(), myline);
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
    let _contents = write_line_to_json(filename.to_string());
}
