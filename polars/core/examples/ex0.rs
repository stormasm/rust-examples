use polars::df;
use polars::prelude::{DataFrame, NamedFrom, Result, Series};

fn test3() {
    let dfd = df!("Fruit" => &["Apple", "Apple", "Pear"],
              "Color" => &["Red", "Yellow", "Green"]);

    let x = dfd.unwrap();
    println!("{:?}", x[0]);
    println!("{:?}", x["Color"])
}

fn main() {
    let dfa = DataFrame::default();
    assert!(dfa.is_empty());
    println!("{:?}", dfa);

    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let dfb: Result<DataFrame> = DataFrame::new(vec![s1, s2]);
    println!("{:?}", dfb.unwrap());

    let dfc: Result<DataFrame> = df!("Fruit" => &["Apple", "Apple", "Pear"],
                                    "Color" => &["Red", "Yellow", "Green"]);
    println!("{:?}", dfc.unwrap());

    test3();
}
