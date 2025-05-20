use std::u8;

pub struct User {
    username: String,
    email: Option<String>,
    age: Option<u8>,
}

impl User {
    pub fn new(username: String) -> Self {
        User {
            username,
            email: None,
            age: None,
        }
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    pub fn get_contact_info(&self) -> String {
        match &self.email {
            Some(email) => format!("username: {}\nemail: {}", self.username, email),
            None => format!("username {}\nemail : None", self.username),
        }
    }

    pub fn is_adult(&self) -> Option<bool> {
        self.age.map(|age| age >= 18) // => banger de pouvoir faire comme ça
    }
}
