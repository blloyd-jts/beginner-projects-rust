use std::io::{self, Write};

// Function to get user input and return it as a String
pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Failed to flush stdout.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input.trim().to_string()
}

// Function to get book details
pub fn get_book_details() -> (String, String, u32, bool) {
    let title = get_input("Enter the title: ");
    let author = get_input("Enter the author: ");
    let year: u32 = loop {
        let year_input = get_input("Enter the publication year: ");
        match year_input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid year (e.g., 1983."),
        };
    };
    let available = get_input("Is the book available? (Yes/No): ")
        .to_lowercase()
        .starts_with('y');

    (title, author, year, available)
}