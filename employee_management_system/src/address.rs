#[derive(Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

impl Address {
    pub fn format(&self) -> String {
        format!("{},\n{}, {} {}", self.street, self.city, self.state, self.zip_code)
    }
}