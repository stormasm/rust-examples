use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).unwrap();
    let contents = read_file(path);
    let map = count_words(contents);
    print(map);
}

fn read_file(path: String) -> String {
    let contents = fs::read_to_string(path).unwrap();
    return contents;
}

fn count_words(contents: String) -> HashMap<String, i32> {
    let split = contents.split(&[' ', '\n'][..]);
    let mut map = HashMap::new();
    for w in split {
        match map.get(w) {
            Some(count) => {
                map.insert(w.to_string(), count + 1);
            }
            None => {
                map.insert(w.to_string(), 1);
            }
        }
    }
    return map;
}

fn print(map: HashMap<String, i32>) {
    println!("Occurences..");

    for (key, value) in map.iter() {
        println!("{key}: {value}");
    }
}
