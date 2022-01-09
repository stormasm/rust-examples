fn f1() {
    let mut vec: Vec<Vec<&str>> = vec![
        ["qwe", "123", "nope"].to_vec(),
        ["qwe", "456", "nope"].to_vec(),
        ["asd", "456", "nope"].to_vec(),
        ["asd", "123", "nope"].to_vec(),
    ];
    vec.sort();
    println!("{:?}", vec)
}

fn main() {
    f1();
}
