use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[allow(dead_code)]
fn read1<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

#[allow(dead_code)]
fn read2<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(
            line?
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}

fn main() -> Result<(), Error> {
    let vec = read1(File::open("file-to-vec.txt")?)?;
    // use `vec` for whatever
    println!("{:?}", vec);

    for x in &vec {
        println!("{}", x);
    }

    Ok(())
}
