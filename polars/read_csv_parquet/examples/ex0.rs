//! ## Read a csv file to a DataFrame
//!use polars::prelude::{CsvEncoding, CsvReader, JsonReader, ParquetReader, SerReader};
//!use polars::prelude::*;

use polars::prelude::{CsvReader, CsvWriter, DataFrame, Result, SerReader, SerWriter};
use std::fs::File;

fn exread() -> Result<DataFrame> {
    // always prefer `from_path` as that is fastest.
    CsvReader::from_path("../datasets/foods1.csv")?
        .has_header(true)
        .finish()
}

fn exwrite(df: &mut DataFrame) -> Result<()> {
    let mut file = File::create("example.csv").expect("could not create file");

    CsvWriter::new(&mut file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(df)
}

fn main() {
    let df = exread();
    println!("{:?}", df);
    exwrite(&mut df.unwrap());
}
