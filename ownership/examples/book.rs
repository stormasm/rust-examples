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
