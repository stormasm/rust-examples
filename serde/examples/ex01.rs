#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate serde;
extern crate serde_json;
use redis::Commands;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    firstname: String,
    lastname: String,
}

fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let mycontact = Contact {
        firstname: "rick".to_string(),
        lastname: "stevens".to_string(),
    };

    let serialized = serde_json::to_string(&mycontact).unwrap();
    println!("Serialized: {}", serialized);
    con.set("rick", serialized)?;

    let k: Option<String> = con.get("rick")?;
    let k1 = k.unwrap();

    let deserialized: Contact = serde_json::from_str(&k1).unwrap();
    println!("Deserialized: {:?}", deserialized);

    Ok(())
}

fn main() {
    println!("Hello, redis!");
    do_something();
}
