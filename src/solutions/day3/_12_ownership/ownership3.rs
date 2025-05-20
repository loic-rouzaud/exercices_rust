struct User {
    name: String,
    age: u8,
}

fn happy_birthday(user: User) -> User {
    User {
        name: user.name,
        age: user.age + 1,
    }
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    let older_user = happy_birthday(user);
    println!("Nom: {}, Âge: {}", older_user.name, older_user.age);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_with_struct() {
        let user1 = User {
            name: String::from("Alice"),
            age: 30,
        };

        let user2 = happy_birthday(user1);

        assert_eq!(user2.name, "Alice");
        assert_eq!(user2.age, 31);

        // TODO: Essaie d'utiliser user1 ici. Pourquoi cela ne fonctionne-t-il pas?
        // Décommente la ligne suivante:
        // println!("Nom de user1: {}", user1.name);
    }
}
