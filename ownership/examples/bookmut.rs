// This is an example from Yehuda Katz's excellent talk on Rust
// https://www.youtube.com/watch?v=uCaYkUmdtPw

pub struct Book {
    title: String,
}

fn main() {
    let title1 = "Gone with the wind".to_string();
    let title2 = "A Harsh Reality".to_string();
    let title4 = "Petes Toothpaste";
    let book1 = Book { title: title1 };
    let book2 = Book { title: title2 };
    let mut book3 = Book {
        title: "Sams Lunch Box".to_string(),
    };
    print_book("Book 1", &book1);
    print_book("Book 2", &book2);
    println!("Book 3 Title: {}", book3.title);
    change_title(&mut book3);
    println!("Book 3 Title: {}", book3.title);
    print_book("Book 3", &book3);
    let mut book4 = Book {
        title: "".to_string(),
    };
    title_book(&mut book4, "Bills Tea Cup");
    print_book("Book 4", &book4);
    title_book(&mut book4, title4);
    print_book("Book 4", &book4);
    change_title(&mut book4);
    print_book("Book 4", &book4);
}

fn print_book(name: &str, book: &Book) {
    println!("{} Title: {}", name, book.title);
}

fn change_title(book: &mut Book) {
    book.title = "Rat Race".to_string();
}

fn title_book(book: &mut Book, title: &str) {
    book.title = title.to_string();
}
