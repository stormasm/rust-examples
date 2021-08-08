use std::str::from_utf8;

fn convert_slice_of_bytes_to_string_slice(v: &[u8]) -> String {
    let result = from_utf8(v).unwrap();
    result.to_string()
}

fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let filecontent = std::fs::read(&filename)?;
    Ok(convert_slice_of_bytes_to_string_slice(&filecontent))
}

fn read_linebuf(linebuf: &String) -> bool {
    let mut iter = linebuf.split_ascii_whitespace();
    if iter.next().unwrap() == "read" {
        let filename = iter.next().unwrap();
        println!("{}", filename);
        let filecontent1 = read_file(&filename);
        println!("{}", filecontent1.unwrap());
        //let filecontent2 = std::fs::read(&filename);
        //println!("{}", filecontent2);
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
