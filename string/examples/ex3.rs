fn pass_along_str(s: &str) -> String {
    println!("{}",s);
    "to buy some apples".into()
}

fn main() -> std::io::Result<()> {
    let s = "john walked to the store ";
    let s1 = pass_along_str(s);
    println!("{}",s1);
    Ok(())
}
