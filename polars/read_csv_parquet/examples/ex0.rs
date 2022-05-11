//! ## Read a csv file to a DataFrame
//!
//!use polars::prelude::{CsvEncoding, CsvReader, JsonReader, ParquetReader, SerReader};
//use polars::prelude::{CsvReader, DataFrame};
use polars::prelude::*;

fn example() -> Result<DataFrame> {
    // always prefer `from_path` as that is fastest.
    CsvReader::from_path("../datasets/foods1.csv")?
        .has_header(true)
        .finish()
}

fn main() {
    let df = example();
    println!("{:?}", df);
}
