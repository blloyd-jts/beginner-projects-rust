use std::io::{self, Write};
use crate::address::Address;

// Function to get user input and return as String
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Failed to flush stdout.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}

// Function to get optional input from the user
pub fn get_optional_input(prompt: &str) -> Option<String> {
    let input = get_input(prompt);
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}

// Function to get address from user
pub fn get_address() -> Option<Address> {
    Some(Address{
        street: get_optional_input("Enter street: ")?,
        city: get_optional_input("Enter city: ")?,
        state: get_optional_input("Enter state: ")?,
        zip_code: get_optional_input("Enter zip code: ")?,
    })
}