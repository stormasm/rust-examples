use polars::prelude::DataFrame;

fn main() {
    let df = DataFrame::default();
    assert!(df.is_empty());
    println!("{:?}", df);
}
