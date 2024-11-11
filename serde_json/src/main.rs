use serde::{Serialize, Deserialize};

//Define the struct you want to convert to/from json
#[derive(Serialize,Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    year: u32
}

impl Book {
    fn new(title: String, author: String, year:u32) -> Book {
        Book {title,author,year}
    }
}

fn main() {
    let book = Book::new(String::from("Rust Programming"), String::from("Jane Doe"), 2021);

    println!("Author of book is : {}", book.author);
    // Serialize the book struct to a json string
    let json_str = serde_json::to_string(&book).expect("Failed to serialize book");
    println!("Serialized JSON: {}", json_str);

    // Deserialize the JSON string back into a Book struct
    let deserialized_book: Book = serde_json::from_str(&json_str).expect("Failed to deserialize JSON");
    println!("Deserialized Book: {:?}", deserialized_book);

}
