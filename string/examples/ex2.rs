/// You only want to pass along str or String
/// And return only String's

fn pass_along_string(s: &mut String) -> String {
    // This works too...
    // You can create a String from a literal string with String::from
    // let tobuy = String::from("to buy some apples");
    let tobuy = "to buy some apples";
    s.push_str(&tobuy);
    s.to_string()
}

fn main() -> std::io::Result<()> {
    // Either one of these will work...
    // let mut s = "john walked to the store ".into();
    let mut s = String::from("john walked to the store ");
    let s1 = pass_along_string(&mut s);
    println!("{}", s1);
    Ok(())
}
