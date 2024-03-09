use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const API: &str = "https://httpbin.org/ip";

    let resp = reqwest::blocking::get(API)?.json::<HashMap<String, String>>()?;
    println!("{resp:#?}");
    Ok(())
}
