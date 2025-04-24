mod library;
mod book;
mod input;

use library::Library;
use input::{get_input, get_book_details};

fn main() {
    let mut library = Library {
        books: Vec::new()
    };

    loop {
        println!("\nWelcome to the Simple Library System!\n");
        println!("Please choose an option: ");
        println!("1.) Add a book");
        println!("2.) List all books");
        println!("3.) Search by Title");
        println!("4.) Search by Author");
        println!("5.) Exit\n");

        let choice = get_input("Enter your choice: ");
        println!("");

        match choice.trim().parse::<u8>() {
            Ok(1) => {
                let (title, author, year, available) = get_book_details();
                library.add_book(title, author, year, available);
            }
            Ok(2) => library.list_books(),
            Ok(3) => {
                let title = get_input("Enter the title: ");
                library.search_by_title(&title);
            }
            Ok(4) => {
                let author = get_input("Enter the author: ");
                library.search_by_author(&author);
            }
            Ok(5) => {
                println!("Exiting....");
                break;
            }
            _ => println!("Option not avialable. Please enter 1, 2, 3, 4, or 5."),
        }
    }
}