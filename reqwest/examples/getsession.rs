use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const API: &str = "https://ws.audioscrobbler.com/2.0/?api_sig=auth.getSession&api_key=dfb1d3ef6c440db993b57701c463e1cd&token=a8bf0d4d177135ac53f410961e3305cf";
    let resp = reqwest::blocking::get(API);
    println!("{resp:#?}");
    Ok(())
}
