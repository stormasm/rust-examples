use polars::prelude::{DataFrame, ParquetReader, Result, SerReader};
use std::env;
use std::fs::File;

fn read(filename: &str) -> Result<DataFrame> {
    //  let r = File::open("../datasets/foods1.parquet").unwrap();
    let r = File::open(filename).unwrap();
    let reader = ParquetReader::new(r);
    reader.finish()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let df = read(&filename).unwrap();
    println!("{:?}", df);
}
