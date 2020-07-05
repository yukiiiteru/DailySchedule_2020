#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        // I cant spell this title
        year: 1979,
    };

    let mut mutabook = immutabook;
    
    borrow_book(&immutabook);

    borrow_book(&mutabook);
    
    new_edition(&mut mutabook);
    
    // new_edition(&mut immutabook);
}

