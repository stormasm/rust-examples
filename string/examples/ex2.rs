fn pass_along_str(s: &mut String) -> String {
    let tobuy = String::from("to buy some apples");
    s.push_str(&tobuy);
    s.to_string()
}

fn main() -> std::io::Result<()> {
    let mut s = "john walked to the store ".into();
    let s1 = pass_along_str(&mut s);
    println!("{}",s1);
    Ok(())
}
