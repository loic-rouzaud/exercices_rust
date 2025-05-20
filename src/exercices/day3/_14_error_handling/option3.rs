// Structure représentant un utilisateur
struct User {
    id: u32,
    username: String,
    email: Option<String>, // L'email est optionnel
    premium: bool,
}

// Structure représentant un système d'utilisateurs
struct UserSystem {
    users: Vec<User>,
}

impl UserSystem {
    // Crée un nouveau système d'utilisateurs vide
    fn new() -> Self {
        todo!();
    }

    // Ajoute un utilisateur au système
    fn add_user(&mut self, user: User) {
        todo!();
    }

    // Trouve un utilisateur par son id
    fn find_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    // TODO: Implémenter une fonction qui trouve l'email d'un utilisateur par son nom d'utilisateur
    // La fonction doit retourner None si l'utilisateur n'existe pas ou si l'email n'est pas défini
    fn find_email_by_username(&self, username: &str) -> Option<&String> {
        // Indice: Utilisez find() pour trouver l'utilisateur puis and_then() pour accéder à l'email
        None // À remplacer par votre implémentation
    }

    // TODO: Implémenter une fonction qui vérifie si un utilisateur est premium par son id
    // La fonction doit retourner None si l'utilisateur n'existe pas,
    // ou Some(true)/Some(false) selon le statut premium
    fn is_premium(&self, id: u32) -> Option<bool> {
        // Indice: Utilisez find() puis map()
        None // À remplacer par votre implémentation
    }
}

fn main() {
    // tester ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_email_by_username() {
        let mut system = UserSystem::new();

        system.add_user(User {
            id: 1,
            username: String::from("test_user"),
            email: Some(String::from("test@example.com")),
            premium: false,
        });

        system.add_user(User {
            id: 2,
            username: String::from("no_email_user"),
            email: None,
            premium: false,
        });

        // User exists with email
        let result1 = system.find_email_by_username("test_user");
        assert_eq!(result1, Some(&String::from("test@example.com")));

        // User exists but has no email
        let result2 = system.find_email_by_username("no_email_user");
        assert_eq!(result2, None);

        // User doesn't exist
        let result3 = system.find_email_by_username("nonexistent");
        assert_eq!(result3, None);
    }

    #[test]
    fn test_is_premium() {
        let mut system = UserSystem::new();

        system.add_user(User {
            id: 1,
            username: String::from("premium_user"),
            email: None,
            premium: true,
        });

        system.add_user(User {
            id: 2,
            username: String::from("regular_user"),
            email: None,
            premium: false,
        });

        // User is premium
        let result1 = system.is_premium(1);
        assert_eq!(result1, Some(true));

        // User is not premium
        let result2 = system.is_premium(2);
        assert_eq!(result2, Some(false));

        // User doesn't exist
        let result3 = system.is_premium(99);
        assert_eq!(result3, None);
    }
}
