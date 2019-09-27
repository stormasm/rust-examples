#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    firstname: String,
    lastname: String
}

fn main() {
    println!("Hello, world!");

    let mycontact = Contact { firstname: "rick".to_string(), lastname: "stevens".to_string() };
    let serialized = serde_json::to_string(&mycontact).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Contact = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
