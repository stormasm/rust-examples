use reedline::{DefaultPrompt, Reedline, Signal};

fn read_file(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filecontent = std::fs::read(&filename)?;
    println!("{:?}", filecontent);
    Ok(())
}

fn read_linebuf(linebuf: &String) -> bool {
    let mut iter = linebuf.split_ascii_whitespace();
    if iter.next().unwrap() == "read" {
        let filename = iter.next().unwrap();
        println!("{}", filename);
        read_file(&filename);
        return true;
    } else {
        return false;
    }
    /*
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("reading file");
        return true
    */
}

fn main() -> std::io::Result<()> {
    let mut line_editor = Reedline::new();

    let prompt = DefaultPrompt::new(1);

    loop {
        let input = line_editor.read_line(&prompt)?;
        match input {
            Signal::Success(s) => {
                if s.trim() == "exit" {
                    break;
                } else if read_linebuf(&s) {
                    println!("processed file");
                } else {
                    println!("{}", s);
                }
            }

            Signal::CtrlC => {
                println!("Ctrl-c");
            }
            Signal::CtrlD => {
                break;
            }
            Signal::CtrlL => {
                line_editor.clear_screen()?;
            }
        }
    }
    Ok(())
}
