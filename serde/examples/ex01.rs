#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate redis;
use redis::Commands;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    firstname: String,
    lastname: String
}


fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let mycontact = Contact { firstname: "rick".to_string(), lastname: "stevens".to_string() };
    let serialized = serde_json::to_string(&mycontact).unwrap();
    println!("Serialized: {}", serialized);
    con.set("rick", serialized)?;
    let json_from_redis = con.get("rick").to_string();

    let deserialized: Contact = serde_json::from_str(&json_from_redis).unwrap();
    println!("Deserialized: {:?}", deserialized);


    Ok(())
}

fn main() {
    println!("Hello, redis!");
    do_something();
}

/*

fn main() {
    println!("Hello, world!");

    let mycontact = Contact { firstname: "rick".to_string(), lastname: "stevens".to_string() };
    let serialized = serde_json::to_string(&mycontact).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
*/
