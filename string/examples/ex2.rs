fn pass_along_str(s: &String) -> String {
    s.to_string()
}

fn main() -> std::io::Result<()> {
    let s = "john walked to the store".into();
    let s1 = pass_along_str(&s);
    println!("{}",s1);
    Ok(())
}
