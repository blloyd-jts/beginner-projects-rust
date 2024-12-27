use std::fs::File;
use std::error::Error;
use csv::{Reader, Writer};
use crate::employee::Employee;
use crate::address::Address;

pub struct CsvOperations;

impl CsvOperations {
    // Save employees to CSV file
    pub fn save_to_csv(employees: &[Employee], filename: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = Writer::from_path(filename)?;
        
        // Write headers
        writer.write_record(&[
            "First Name", "Last Name", "Middle Name", "Department", "Title", 
            "Email", "Phone", "Street", "City", "State", "Zip Code"
        ])?;

        // Write employee data
        for employee in employees {
            let address = employee.address.as_ref();
            let record = vec![
                employee.first_name.as_str(),
                employee.last_name.as_str(),
                employee.middle_name.as_deref().unwrap_or(""),
                employee.department.as_deref().unwrap_or(""),
                employee.title.as_deref().unwrap_or(""),
                employee.email.as_deref().unwrap_or(""),
                employee.phone.as_deref().unwrap_or(""),
                address.map(|a| a.street.as_str()).unwrap_or(""),
                address.map(|a| a.city.as_str()).unwrap_or(""),
                address.map(|a| a.state.as_str()).unwrap_or(""),
                address.map(|a| a.zip_code.as_str()).unwrap_or("")
            ];
            writer.write_record(&record)?;
        }
        
        writer.flush()?;
        Ok(())
    }

    // Load employees from CSV file
    pub fn load_from_csv(filename: &str) -> Result<Vec<Employee>, Box<dyn Error>> {
        let file = File::open(filename);
        
        // If file doesn't exist, return empty vector
        if file.is_err() {
            return Ok(Vec::new());
        }

        let mut reader = Reader::from_reader(file?);
        let mut employees = Vec::new();

        for result in reader.records() {
            let record = result?;
            
            // Create address if any address fields are present
            let address = if !record[7].is_empty() || !record[8].is_empty() || 
                         !record[9].is_empty() || !record[10].is_empty() {
                Some(Address {
                    street: record[7].to_string(),
                    city: record[8].to_string(),
                    state: record[9].to_string(),
                    zip_code: record[10].to_string(),
                })
            } else {
                None
            };

            // Create employee from record
            let employee = Employee::new(
                record[0].to_string(),
                record[1].to_string(),
                if record[2].is_empty() { None } else { Some(record[2].to_string()) },
                if record[3].is_empty() { None } else { Some(record[3].to_string()) },
                if record[4].is_empty() { None } else { Some(record[4].to_string()) },
                if record[5].is_empty() { None } else { Some(record[5].to_string()) },
                if record[6].is_empty() { None } else { Some(record[6].to_string()) },
                address,
            );
            
            employees.push(employee);
        }

        Ok(employees)
    }
}