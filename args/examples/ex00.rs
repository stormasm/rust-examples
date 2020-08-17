fn main() {
    let args: Vec<_> = std::env::args().collect();

//  Figure out how to pass in the correct value
    let width = args[1].parse::<usize>().expect("Need a width in columns");
//  let width = 20000;
    process(width);
}

fn process(width: usize) {
    assert_eq!(width, 2);
}

// pub fn draw_table(table: &Table, termwidth: usize) {
