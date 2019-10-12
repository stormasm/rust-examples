use redis::Commands;
use serde_json::json;
use serde_json::{Result, Value};
use std::string::String;

fn string_to_json() -> Result<(Value)> {
    let data = json!(["21195107", "21190487", "21189256", "21193497", "21191588"]);
    Ok(data)
}

fn write_json_to_redis(json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
    // Convert the serde value to a Vector
    let vec = json.as_array().unwrap();

    // iterate over the vector
    for i in 0..vec.len() {
        // you must convert &str to String
        let vy = &vec[i].as_str().unwrap().to_string();
        redis::cmd("SADD").arg("setbill").arg(vy).execute(&mut con);
    }

    let myid = String::from("999");
    con.set("rick", myid)?;

    let k: Option<String> = con.get("rick")?;
    let k1 = k.unwrap();
    redis::cmd("SADD").arg("setpete").arg(k1).execute(&mut con);

    let x55 = String::from("55");
    redis::cmd("SADD").arg("setpete").arg(x55).execute(&mut con);
    Ok(())
}

fn main() {
    let json = string_to_json().unwrap();
    let _x = write_json_to_redis(json);
}
