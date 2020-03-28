fn my_fn(arg1: &Option<Box<i32>>) -> i32 {
    if arg1.is_none() {
        return 0;
    }
    let integer = arg1.unwrap();
    *integer
}

fn main() {
    let integer = 42;
    my_fn(&Some(Box::new(integer)));
}
