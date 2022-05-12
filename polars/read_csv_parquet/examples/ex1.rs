//! ## Read a csv file to a DataFrame
//!use polars::prelude::{CsvEncoding, CsvReader, JsonReader, ParquetReader, SerReader};
//!use polars::prelude::*;

use polars::prelude::{
    CsvReader, CsvWriter, DataFrame, ParquetReader, ParquetWriter, Result, SerReader, SerWriter,
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

fn stringify() -> String {
    format!("error code:")
}

fn write(df: &mut DataFrame) -> Result<()> {
    let mut file = File::create("example.parquet").expect("could not create file");

    //    ParquetWriter::new(file).finish(df.as_mut())?;
    ParquetWriter::new(file).finish(df)?;

    Ok(())
}

fn main() {
    let mut df = read().unwrap();
    println!("{:?}", df);
    let _ = write(&mut df);
}
