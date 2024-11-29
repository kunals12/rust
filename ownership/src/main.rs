// Define a struct to represent a book
struct Book {
    title: String,  // The title of the book
    author: String, // The author of the book
    pages: u32,     // Number of pages in the book
}

fn main() {
    // Create an instance of the Book struct
    let book = Book {
        title: String::from("Rust Programming"), // Initialize the title
        author: String::from("Jane Doe"),       // Initialize the author
        pages: 320,                             // Initialize the page count
    };

    // Borrow the book to display its details
    // Ownership is not transferred, only a reference is passed
    display_book(&book); 

    // Borrow the book again to get the page count
    let pages = book_pages(&book); 

    // Since ownership is not transferred, we can still use `book` here
    println!("The book '{}' has {} pages.", book.title, pages);
}

// Function to display book details
// Takes a reference to a Book as an argument to avoid taking ownership
fn display_book(book: &Book) {
    // Access fields of the borrowed book
    println!("Title: {}, Author: {}", book.title, book.author);
}

// Function to get the number of pages in the book
// Also borrows the book to avoid transferring ownership
fn book_pages(book: &Book) -> u32 {
    book.pages // Return the number of pages
}
