//! ## Read a csv file to a DataFrame
//!use polars::prelude::{CsvEncoding, CsvReader, JsonReader, ParquetReader, SerReader};
//!use polars::prelude::*;

use polars::prelude::{
    CsvReader, CsvWriter, DataFrame, ParquetReader, Result, SerReader, SerWriter,
};
use std::fs::File;

fn read() -> Result<DataFrame> {
    // always prefer `from_path` as that is fastest.
    //ParquetReader::from_path("../datasets/foods1.parquet")?
    //    .has_header(true)
    //    .finish();

    let r = File::open("../datasets/foods1.parquet").unwrap();
    let reader = ParquetReader::new(r);
    reader.finish()
}

/*
fn write(df: &mut DataFrame) -> Result<()> {
    let mut file = File::create("example.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(df)
}
*/

fn main() {
    let mut df = read().unwrap();
    println!("{:?}", df);
    //let _ = write(&mut df);
}
