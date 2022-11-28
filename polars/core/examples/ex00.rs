use polars::prelude::{DataFrame, NamedFrom, PolarsResult, Series};

fn test1() {
    let df = DataFrame::default();
    assert!(df.is_empty());
    println!("{:?}", df);
}

fn test2() -> PolarsResult<()> {
    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let df = DataFrame::new(vec![s1, s2])?;
    println!("{:?}", df);
    Ok(())
}

/*
fn test3() {
    let df: Result<DataFrame> = df!("Fruit" => &["Apple", "Apple", "Pear"],
              "Color" => &["Red", "Yellow", "Green"]);

    let x = df.unwrap();
    println!("{:?}", x[0]);
    println!("{:?}", x["Color"])
}
*/

fn main() {
    test1();
    let _x = test2();
    // test3();
}
