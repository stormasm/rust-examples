// This is an example from Yehuda Katz's excellent talk on Rust
// https://www.youtube.com/watch?v=uCaYkUmdtPw

pub struct Book {
    title: String
}

fn main() {
    let title = "Gone with the wind".to_string();
    let title4 = "Petes Toothpaste";
    let book = Book {title};
    //let title2 = "Sam's Lunch Box".to_string();
    //let mut book2 = Book {"Sams Lunch Box"};
    let mut book3 = Book {title: "Sams Lunch Box".to_string()};
    //title_book(&mut book3,"Bills Tea Cup");
    print_book(&book);
    //println!("{}",book.title);
    println!("Book 3 title = {}",book3.title);
    change_title(&mut book3);
    println!("Book 3 title = {}",book3.title);
    print_book(&book3);
    let mut book4 = Book {title: "".to_string()};
    title_book(&mut book4,"Bills Tea Cup");
    print_book(&book4);
    title_book(&mut book4,title4);
    print_book(&book4);
    change_title(&mut book4);
    print_book(&book4);
}

fn print_book(book: &Book) {
    println!("Title: {}",book.title);
}

fn change_title(book: &mut Book) {
    book.title = "Rat Race".to_string();
}

fn title_book(book: &mut Book, title: &str) {
    book.title = title.to_string();
}
