fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PARAM_STR: &str = "api_keydfb1d3ef6c440db993b57701c463e1cdmethodauth.getSessiontoken6853c9d79a2aa9e15a16f9289965dd03mysecret";
    let signature = md5::compute(PARAM_STR);
    let signature = format!("{:x}", signature);
    println!("{signature:#?}");
    Ok(())
}
