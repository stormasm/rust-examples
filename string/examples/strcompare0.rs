// https://turreta.com/2019/09/07/rust-how-to-compare-strings/

fn main() -> std::io::Result<()> {
    let apple: String = String::from("apple");
    let banana: String = String::from("banana");
    println!("{}", apple.eq(&banana));
    let apple1: String = String::from("apple");
    println!("{}", apple.eq(&apple1));

    let apple2: &str = "apple";
    let banana2: &str = "banana";
    println!("{}", apple2.eq(banana2));
    println!("{}", banana2.eq("banana"));

    Ok(())
}
