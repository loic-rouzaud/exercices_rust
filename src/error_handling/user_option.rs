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
        // À implémenter - Retourne le username et l'email s'il existe
        match &self.email {
            Some(email) => format!("username: {}\nemail: {}", self.username, email),
            None => format!("username {}\nemail : None", self.username),
        }
    }

    pub fn is_adult(&self) -> Option<bool> {
        // À implémenter - Retourne Some(true) si l'âge est présent et >= 18
        // Some(false) si l'âge est présent et < 18
        // None si l'âge n'est pas renseigné
        self.age.map(|age| age >= 18) // => banger de pouvoir faire comme ça
    }
}
