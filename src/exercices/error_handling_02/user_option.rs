use std::u8;

// Exercice difficile

pub struct User {
    username: String,
    email: Option<String>,
    age: Option<u8>,
}

impl User {
    pub fn new(username: String) -> Self {
        // TODO() : Implémenter la fonction new qui crée un nouvel utilisateur
        // avec un nom d'utilisateur, mais sans email ni âge (None)
    }

    pub fn with_email(mut self, email: String) -> Self {
        // TODO() : Implémenter la méthode with_email qui définit l'email de l'utilisateur
        // et retourne self pour permettre le chaînage des méthodes
    }

    pub fn with_age(mut self, age: u8) -> Self {
        // TODO() : Implémenter la méthode with_age qui définit l'âge de l'utilisateur
        // et retourne self pour permettre le chaînage des méthodes
    }

    pub fn get_contact_info(&self) -> String {
        // TODO() : Implémenter la méthode get_contact_info qui retourne une chaîne formatée
        // contenant le nom d'utilisateur et l'email s'il existe
        // Utiliser le pattern matching pour gérer le cas où l'email est None
    }

    pub fn is_adult(&self) -> Option<bool> {
        // TODO() : Implémenter la méthode is_adult qui retourne:
        // - Some(true) si l'âge est présent et >= 18
        // - Some(false) si l'âge est présent et < 18
        // - None si l'âge n'est pas renseigné
        // Utiliser la méthode map sur Option pour une solution élégante
    }
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(String::from("alice"));
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, None);
        assert_eq!(user.age, None);
    }

    #[test]
    fn test_with_email_and_age() {
        let user = User::new(String::from("bob"))
            .with_email(String::from("bob@example.com"))
            .with_age(25);

        assert_eq!(user.username, "bob");
        assert_eq!(user.email, Some(String::from("bob@example.com")));
        assert_eq!(user.age, Some(25));
    }

    #[test]
    fn test_get_contact_info() {
        let user1 = User::new(String::from("carol")).with_email(String::from("carol@example.com"));
        let user2 = User::new(String::from("dave"));

        assert_eq!(user1.get_contact_info(), "carol (carol@example.com)");
        assert_eq!(user2.get_contact_info(), "dave (email not provided)");
    }

    #[test]
    fn test_is_adult() {
        let adult = User::new(String::from("eve")).with_age(25);
        let minor = User::new(String::from("frank")).with_age(15);
        let unknown = User::new(String::from("grace"));

        assert_eq!(adult.is_adult(), Some(true));
        assert_eq!(minor.is_adult(), Some(false));
        assert_eq!(unknown.is_adult(), None);
    }
}
