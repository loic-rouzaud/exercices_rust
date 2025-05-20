struct User {
    id: u32,
    username: String,
    email: Option<String>,
    premium: bool,
}

struct UserSystem {
    users: Vec<User>,
}

impl UserSystem {
    fn new() -> Self {
        UserSystem { users: Vec::new() }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn find_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }

    fn find_email_by_username(&self, username: &str) -> Option<&String> {
        self.users
            .iter()
            .find(|user| user.username == username)
            .and_then(|user| user.email.as_ref())
    }

    fn is_premium(&self, id: u32) -> Option<bool> {
        self.users
            .iter()
            .find(|user| user.id == id)
            .map(|user| user.premium)
    }
}

fn main() {
    let mut system = UserSystem::new();

    system.add_user(User {
        id: 1,
        username: String::from("alice"),
        email: Some(String::from("alice@example.com")),
        premium: true,
    });

    system.add_user(User {
        id: 2,
        username: String::from("bob"),
        email: None,
        premium: false,
    });

    system.add_user(User {
        id: 3,
        username: String::from("charlie"),
        email: Some(String::from("charlie@example.com")),
        premium: false,
    });

    match system.find_email_by_username("alice") {
        Some(email) => println!("Email d'Alice: {}", email),
        None => println!("Pas d'email pour Alice"),
    }

    match system.find_email_by_username("bob") {
        Some(email) => println!("Email de Bob: {}", email),
        None => println!("Pas d'email pour Bob"),
    }

    match system.is_premium(1) {
        Some(true) => println!("L'utilisateur 1 est premium"),
        Some(false) => println!("L'utilisateur 1 n'est pas premium"),
        None => println!("Utilisateur 1 non trouvé"),
    }
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
