// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

//use std::io;
use std::io::BufRead;
//use std::io::BufWriter;
// use std::io::Write;

use std::convert::TryInto;

#[derive(Debug)]
struct FileToVec<'a> {
    key: &'a mut Vec<u32>,
    value: &'a mut Vec<String>,
}

impl<'a> FileToVec<'a> {
    fn is_even(num: u32) -> bool {
        (num) & 1 == 0
    }

    fn readfile(&mut self, filename: String) {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);
        //let mut writer = BufWriter::new(io::stdout());

        for (mynum, myline) in file.lines().enumerate() {
            let xline = myline.unwrap().clone();
            if FileToVec::is_even(mynum.try_into().unwrap()) {
                let xint = xline.parse::<u32>().unwrap();
                //writeln!(writer, "{0}\n", xint).unwrap();
                self.key.push(xint);
            }
            if !FileToVec::is_even(mynum.try_into().unwrap()) {
                //writeln!(writer, "{0}\n", xline);
                self.value.push(xline);
            }
        }

        for i in 0..self.key.len() {
            println!("{} {}", self.key[i], self.value[i]);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    // Instantiate a FileToVec
    let mut ftv: FileToVec = FileToVec {
        key: &mut Vec::new(),
        value: &mut Vec::new(),
    };

    let _contents = FileToVec::readfile(&mut ftv, filename.to_string());
}
