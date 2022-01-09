fn f1() {
    let mut vec: Vec<u32> = vec![45, 3, 6, 2];
    vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}",vec)
}

fn main() {
    f1()
}
