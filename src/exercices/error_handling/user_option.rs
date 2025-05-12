use std::u8;

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
