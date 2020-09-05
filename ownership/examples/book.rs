pub struct Book {
    title: String
}

fn main() {
    let title = "Gone with the wind".to_string();
    let book = Book {title};
    println!("{:?}",book.title);
}
