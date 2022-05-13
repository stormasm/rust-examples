use polars::prelude::{DataFrame, NamedFrom, Result, Series};

fn main() {
    let dfa = DataFrame::default();
    assert!(dfa.is_empty());
    println!("{:?}", dfa);

    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let dfb: Result<DataFrame> = DataFrame::new(vec![s1, s2]);
    println!("{:?}", dfb.unwrap());
}
