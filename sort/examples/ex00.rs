fn f2() {
    let mut vec: Vec<u32> = vec![45, 3, 6, 2];
    vec.sort();
    println!("{:?}", vec)
}

// This switches around the order in which elements are compared,
// so that smaller elements appear larger
// to the sorting function and vice versa.

fn f1() {
    let mut vec: Vec<u32> = vec![45, 3, 6, 2];
    vec.sort_by(|a, b| b.cmp(a));
    println!("{:?}", vec)
}

fn main() {
    f1();
    f2()
}
