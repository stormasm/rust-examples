use urlencoding::encode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let encoded = encode("auth.getSession");
    println!("{}", encoded);
    Ok(())
}
