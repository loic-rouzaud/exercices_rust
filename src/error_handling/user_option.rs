use std::u8;

struct User {
    username: String,
    email: Option<String>,
    age: Option<u8>,
}

impl User {
    fn new(username: String) -> Self {
        User {
            username,
            email: Some(String::from("")),
            age: Some(0),
        }
    }

    fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    fn with_age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    fn get_contact_info(&self) -> String {
        // À implémenter - Retourne le username et l'email s'il existe
        match &self.email {
            Some(email) => email.to_string(),
            None => String::from("None"),
        }
    }

    fn is_adult(&self) -> Option<bool> {
        // À implémenter - Retourne Some(true) si l'âge est présent et >= 18
        // Some(false) si l'âge est présent et < 18
        // None si l'âge n'est pas renseigné
        self.age.map(|age| age >= 18) // => banger de pouvoir faire comme ça
    }
}
