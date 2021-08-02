use reedline::{DefaultPrompt, Reedline, Signal};

fn main() -> std::io::Result<()> {
    let mut line_editor = Reedline::new();

    let prompt = DefaultPrompt::new(1);

    loop {
        let input = line_editor.read_line(&prompt)?;
        match input {
            Signal::Success(s) => {
                if s.trim() == "exit" {
                    break;
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
