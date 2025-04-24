mod employee;
mod address;
mod input;
mod csv_operations;

use employee::Employee;
use input::{get_input, get_optional_input, get_address};
use csv_operations::CsvOperations;

const CSV_FILE: &str = "employees.csv";

fn main() {
    // Load existing employees from CSV file
    let mut employees: Vec<Employee> = match CsvOperations::load_from_csv(CSV_FILE) {
        Ok(emps) => emps,
        Err(e) => {
            println!("Error loading employees: {}. Starting with empty list.", e);
            Vec::new()
        }
    };

    loop {
        println!("\nWelcome to the Employee Management system!\n");
        println!("Please choose an option:");
        println!("1.) Add an employee ");
        println!("2.) Update an employee: ");
        println!("3.) List all employees: ");
        println!("4.) Remove an employee ");
        println!("5.) Save to CSV");
        println!("6.) Exit\n");

        let choice = get_input("Enter your choice: ");
        println!("");

        match choice.trim().parse::<u8>() {
            Ok(1) => {
                // Add an employee
                println!("\n* * * REQUIRED * * *");
                let first_name = get_input("Enter the employee's first name: ");
                let last_name = get_input("Enter the employee's last name: ");
                println!("* * * OPTIONAL * * *");
                let middle_name = get_optional_input("Enter the employee's middle name: ");
                let department = get_optional_input("Enter the employee's department: ");
                let title = get_optional_input("Enter the employee's title: ");
                let email = get_optional_input("Enter the employee's email address: ");
                let phone = get_optional_input("Enter the employee's phone number: ");
                let address = get_address();

                let employee = Employee::new(
                    first_name,
                    last_name,
                    middle_name,
                    department,
                    title,
                    email,
                    phone,
                    address,
                );
                employees.push(employee);

                // Save after adding
                if let Err(e) = CsvOperations::save_to_csv(&employees, CSV_FILE) {
                    println!("\nError saving to CSV: {}", e);
                }

                println!("\nSuccessfully added an employee!\n");
            }
            Ok(2) => {
                // Updating an employee
                let first_name = get_input("Enter the first name of the employee to update: ");
                let last_name = get_input("Enter the last name of the employee to update: ");

                if let Some(employee) = employees.iter_mut().find(|e| {
                    e.first_name.to_lowercase() == first_name.to_lowercase()
                        && e.last_name.to_lowercase() == last_name.to_lowercase()
                }) {
                    let department = get_optional_input("Enter the new department or leave blank to keep current: ");
                    let title = get_optional_input("Enter the new title or leave blank to keep current: ");
                    let email = get_optional_input("Enter the new email address or leave blank to keep current: ");
                    let phone = get_optional_input("Enter the new phone number or leave blank to keep current: ");
                    let address = get_address();

                    employee.update_contact_info(department, title, email, phone, address);

                    // Save after updating
                    if let Err(e) = CsvOperations::save_to_csv(&employees, CSV_FILE) {
                        println!("\nError saving to CSV: {}", e);
                    }

                    println!("\nSuccessfully updated employee's contact info!\n");
                } else {
                    println!("\nEmployee not found.\n");
                }
            }
            Ok(3) => {
                // Listing all employees
                if employees.is_empty() {
                    println!("No employees found.");
                } else {
                    println!("* * * * * EMPLOYEES * * * * *");
                    for employee in &employees {
                        employee.display();
                    }
                }
            }
            Ok(4) => {
                // Removing an employee
                let first_name = get_input("Enter the first name of the employee to remove: ");
                let last_name = get_input("Enter the last name of the employee to remove: ");

                let initial_length = employees.len();
                employees.retain(|e| {
                    !(e.first_name.to_lowercase() == first_name.to_lowercase()
                        && e.last_name.to_lowercase() == last_name.to_lowercase())
                });

                if employees.len() < initial_length {
                    // Save after removing
                    if let Err(e) = CsvOperations::save_to_csv(&employees, CSV_FILE) {
                        println!("\nError saving to CSV: {}", e);
                    }
                    println!("\nSuccessfully removed an employee!\n");
                } else {
                    println!("\nEmployee not found.\n");
                }
            }
            Ok(5) => {
                // Manual save to CSV
                match CsvOperations::save_to_csv(&employees, CSV_FILE) {
                    Ok(_) => println!("\nSuccessfully saved employees to CSV file!\n"),
                    Err(e) => println!("\nError saving to CSV: {}\n", e),
                }
            }
            Ok(6) => {
                // Save before exiting
                if let Err(e) = CsvOperations::save_to_csv(&employees, CSV_FILE) {
                    println!("\nError saving to CSV: {}", e);
                }
                println!("Exiting.....");
                break;
            }
            _ => println!("Option not available. Please choose 1, 2, 3, 4, 5, or 6."),
        }
    }
}