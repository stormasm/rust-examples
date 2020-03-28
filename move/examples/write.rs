use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::prelude::*;
use csv::Reader;
use infcsv::point::Point;

fn lp_writer(filename: &str, vec: Vec<Point>) -> Result<(), Box<dyn Error>> {
    println!("{}", filename);
    for entry in vec.iter() {
        println!("{:?}\n", entry);
        let lp = entry.get_lineprotocol();
        //println!("{:?}\n", lp);
    }
    Ok(())
}

fn csv_reader(filename: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filename)?;
    let mut vecp: Vec<Point> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let timestamp = &record[0];
        let close = &record[4];
        let volume = &record[5];
        let t1 = Utc.datetime_from_str(timestamp, "%Y-%m-%d %H:%M").unwrap();
        let point: Point = Point {
            measurement: file_stem(filename).unwrap().to_string(),
            tagset: Point::set_tagset(),
            fieldset: Point::set_fieldset(volume.to_string(), close.to_string()),
            timestamp: t1.timestamp().to_string(),
        };
        vecp.push(point);
    }
    //println!("{:?}\n", vecp);
    Ok(vecp)
}

fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    let mut vec: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        vec.push(path);
    }
    Ok(vec)
}

fn file_stem(filename: &str) -> Option<&str> {
    let path = Path::new(filename);
    let name = path.file_stem().unwrap().to_str();
    name
}

// https://doc.rust-lang.org/std/path/struct.Path.html#method.join
// Concat the filename and the extension
fn create_filename(filename: &str, extension: &str) -> std::path::PathBuf {
    let x = Path::new(filename).with_extension(extension);
    x
}

fn write_processor(dirin: String, dirout: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(dirin).unwrap();
    for name in vec {
        let filename = name.to_str().unwrap();
        let vecp = csv_reader(filename);

        let stem = file_stem(filename).unwrap();
        let fn1 = create_filename(stem,"txt");

        let fn2 = Path::new(&dirout).join(fn1);
        let x = fn2.to_str().unwrap();

        let _ = lp_writer(&x, vecp.unwrap());
    }
    Ok(())
}

fn main() {
    let dirin = String::from("./examples/data/csv");
    let dirout = String::from("./examples/data/out");
    let _ = write_processor(dirin, dirout);
}
