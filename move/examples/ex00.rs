fn my_fn(arg1: &Option<Box<i32>>) -> i32 {
    match arg1 {
        Some(b) => **b,
        None => 0,
    }
}

fn main() {
    let integer = 42;
    let x = my_fn(&Some(Box::new(integer)));
    println!("{}",x);
}
