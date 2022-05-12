use polars::prelude::{DataFrame, ParquetReader, ParquetWriter, Result, SerReader};
use std::fs::File;

fn read() -> Result<DataFrame> {
    let r = File::open("../datasets/foods1.parquet").unwrap();
    let reader = ParquetReader::new(r);
    reader.finish()
}

fn write(df: &mut DataFrame) -> Result<()> {
    let file = File::create("../datasets/foods1a.parquet").expect("could not create file");
    ParquetWriter::new(file).finish(df)?;
    Ok(())
}

fn main() {
    let mut df = read().unwrap();
    println!("{:?}", df);
    let _ = write(&mut df);
}
