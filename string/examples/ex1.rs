use std::str::from_utf8;

fn convert_slice_of_bytes_to_string_slice(v: &[u8]) {
    let result = from_utf8(v).unwrap();
    println!("{}",result);
}

fn read_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filecontent = std::fs::read(&filename)?;
    convert_slice_of_bytes_to_string_slice(&filecontent);
    Ok(())
}

fn read_linebuf(linebuf: &String) -> bool {
    let mut iter = linebuf.split_ascii_whitespace();
    if iter.next().unwrap() == "read" {
        let filename = iter.next().unwrap();
        println!("{}", filename);
        let _tmp = read_file(&filename);
        let filecontent = std::fs::read(&filename);
        // println!("{:?}", filecontent);
        return true;
    } else {
        return false;
    }
}

fn main() -> std::io::Result<()> {
    let s = "read file1.txt".into();
    read_linebuf(&s);
    Ok(())
}
