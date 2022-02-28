use redis::Commands;
use serde_json::json;
use serde_json::{Result, Value};
use std::string::String;

fn string_to_json1() -> Result<Value> {
    let data = json!(["21195107", "21190487", "21189256", "21193497", "21191588"]);
    Ok(data)
}

fn string_to_json2() -> Result<Value> {
    let data = json!({"firstname":"rick","lastname":"stevens"});
    Ok(data)
}

fn write_json_to_redis_key(key: String, json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    // Convert the serde value to a std::string::String
    let jsonstring = json.to_string();

    let _: () = con.set(key, jsonstring)?;

    Ok(())
}

fn write_json_to_redis_set(key: String, json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    // Convert the serde value to a std::string::String
    let jsonstring = json.to_string();

    redis::cmd("SADD")
        .arg(key)
        .arg(jsonstring)
        .execute(&mut con);
    Ok(())
}

fn main() {
    let json1 = string_to_json1().unwrap();
    let json2 = string_to_json2().unwrap();

    let key1 = "key1set".to_string();
    let _x1 = write_json_to_redis_set(key1, json1);

    let key2 = "key2set".to_string();
    let _x2 = write_json_to_redis_set(key2, json2);

    let json3 = json!(["bill", "pete", "paul"]);
    let json4 = json!({"firstname":"mike","lastname":"roth"});

    let key3 = "key3".to_string();
    let _x3 = write_json_to_redis_key(key3, json3);

    let key4 = "key4".to_string();
    let _x4 = write_json_to_redis_key(key4, json4);
}
