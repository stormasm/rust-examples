#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    firstname: String,
    lastname: String,
}

fn main() {
    println!("Hello, world!");

    let mycontact = Contact {
        firstname: "rick".to_string(),
        lastname: "stevens".to_string(),
    };
    let jsondata = serde_json::to_string(&mycontact).unwrap();
    println!("Serialized: {}", jsondata);

    let contact1: Contact = serde_json::from_str(&jsondata).unwrap();
    println!("Deserialized: {:?}", contact1);
}
