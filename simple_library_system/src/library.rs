use crate::book::Book;

pub struct Library {
    pub books: Vec<Book>,
}

impl Library {

    // Function to add a new book to the library
    pub fn add_book(&mut self, title: String, author: String, year: u32, available: bool) {
        let new_book = Book {
            title,
            author,
            year,
            available,
        };
        self.books.push(new_book);
        println!("Book added!");
    }

    // Function to list all books in the library
    pub fn list_books(&self) {
        if self.books.is_empty() {
            println!("There are no books in the library.");
        } else {
            println!("Listing all books....\n");
            for (index, book) in self.books.iter().enumerate() {
                println!("{}. {} by {}\nPublished: {}\nAvailable: {}\n", index +1, book.title, book.author, book.year, if book.available {"Yes"} else {"No"});
            }
        }
    }

    // Function to search the library for books with a specified title
    pub fn search_by_title(&self, title: &str) {
        let mut found = false;

        for book in &self.books {
            if book.title.to_lowercase() == title.to_lowercase() {
                println!("\nFound: {} by {}\nPublished: {}\nAvailable: {}", book.title, book.author, book.year, if book.available {"Yes"} else {"No"});
                found = true;
                break;
            }
        }
        if !found {
            println!("No book found with the title {}.", title);
        }
    }

    // Function to search the library for books by a specified author
    pub fn search_by_author(&self, author: &str) {
        let mut found = false;

        for book in &self.books {
            if book.author.to_lowercase().contains(&author.to_lowercase()) {
                println!("\nFound: {} by {}\nPublished: {}\nAvailable: {}", book.title, book.author, book.year, if book.available {"Yes"} else {"No"});
                found = true;
            }
        }
        if !found {
            println!("No book found by {}.", author);
        }
    }
}