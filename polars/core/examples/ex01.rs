use polars::datatypes::Float64Chunked;
use polars::datatypes::UInt32Chunked;
use polars::prelude::{DataFrame, IntoSeries, NamedFrom};

fn main() {
    let a = UInt32Chunked::new("a", &[1, 2, 3]).into_series();
    let b = Float64Chunked::new("b", &[10., 8., 6.]).into_series();
    let ab = DataFrame::new(vec![a, b]).unwrap();
    println!("{:?}", ab);
}
