use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

use std::convert::TryInto;

use r2d2_redis::{r2d2, RedisConnectionManager};
// use redis::Commands;
use redis::{Commands, RedisResult};

#[allow(dead_code)]
fn write_json_to_redis(key: String, value: String) -> RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let _x0 = redis::cmd("HSET")
        .arg("hn-story-19-bak")
        .arg(key)
        .arg(value)
        .query::<u64>(&mut *con)
        .unwrap();

    Ok(())
}

fn is_even(num: u32) -> bool {
    (num) & 1 == 0
}

fn read_file_to_buffer2(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let mut vec_key = Vec::new();
    let mut vec_value = Vec::new();
    let mut myline = String::new();

    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        myline = line.unwrap().clone();
        println!("{}", myline);
        if is_even(num.try_into().unwrap()) {
            // writeln!(writer, "{0}\n", num).unwrap();
            vec_key.push(&myline);
        }
        if !is_even(num.try_into().unwrap()) {
            vec_value.push(&myline);
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

    let _contents = read_file_to_buffer2(filename.to_string());

    // let _ = write_json_to_redis(item_id.to_string(), item_json);
}
