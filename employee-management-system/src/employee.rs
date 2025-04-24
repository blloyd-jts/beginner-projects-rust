use crate::address::Address;

pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<Address>,
}

impl Employee {
    // Function to create a new employee
    pub fn new(
        first_name: String,
        last_name: String,
        middle_name: Option<String>,
        department: Option<String>,
        title: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        address: Option<Address>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            middle_name,
            department,
            title,
            email,
            phone,
            address,
        }
    }

    // Function to update employee contact info
    pub fn update_contact_info(
        &mut self,
        department: Option<String>,
        title: Option<String>,
        email: Option<String>,
        phone: Option<String>,
        address: Option<Address>,
    ) {
        if let Some(dept) = department {
            self.department = Some(dept);
        }
        if let Some(title) = title {
            self.title = Some(title);
        }
        if let Some(email) = email {
            self.email = Some(email);
        }
        if let Some(phone) = phone {
            self.phone = Some(phone);
        }
        if let Some(address) = address {
            self.address = Some(address);
        }
    }

    // Function to format phone number as: (XXX) XXX-XXXX
    fn format_phone(&self) -> String {
        match &self.phone {
            Some(phone) if phone.len() == 10 => {
                format!("({}) {}-{}", &phone[0..3], &phone[3..6], &phone[6..10])
            }
            _ => "N/A".to_string()
        }
    }


    // Function to display the employees info
    pub fn display(&self) {
        println!("\n{}{}{}\n", self.first_name, match &self.middle_name {
            Some(middle) => format!(" {} ", middle),
            None => " ".to_string(),
        }, self.last_name);

        match &self.department {
            Some(dept) => println!("Department: {}", dept),
            None => println!("Department: N/A"),
        }

        match &self.title {
            Some(title) => println!("Title: {}", title),
            None => println!("Title: N/A"),
        }

        match &self.email {
            Some(email) => println!("Email: {}", email),
            None => println!("Email: N/A"),
        }

        println!("Phone: {}", self.format_phone());

        match &self.address {
            Some(address) => println!("Address: {}", address.format()),
            None => println!("Address: N/A"),
        }

        println!("");
        println!("* * * * * * * * * * * * * * * *");
    }
}