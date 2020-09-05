// This is an example from Yehuda Katz's excellent talk on Rust
// https://www.youtube.com/watch?v=uCaYkUmdtPw

pub struct Book {
    title: String
}

fn main() {
    let title = "Gone with the wind".to_string();
    let book = Book {title};
    print_book(&book);
    println!("{}",book.title);
}

fn print_book(book: &Book) {
    println!("Title: {}",book.title);
}
