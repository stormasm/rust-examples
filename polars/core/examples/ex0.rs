use polars::df;
use polars::prelude::{DataFrame, NamedFrom, Result, Series};

fn test1() {
    let df = DataFrame::default();
    assert!(df.is_empty());
    println!("{:?}", df);
}

fn test2() {
    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let df: Result<DataFrame> = DataFrame::new(vec![s1, s2]);
    println!("{:?}", df.unwrap());
}

fn test3() {
    let df: Result<DataFrame> = df!("Fruit" => &["Apple", "Apple", "Pear"],
              "Color" => &["Red", "Yellow", "Green"]);

    let x = df.unwrap();
    println!("{:?}", x[0]);
    println!("{:?}", x["Color"])
}

fn main() {
    test1();
    test2();
    test3();
}
