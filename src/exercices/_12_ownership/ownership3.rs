struct User {
    name: String,
    age: u8,
}

// TODO: Implémenter cette fonction pour qu'elle renvoie un nouveau User
// avec le même nom mais un âge incrémenté d'un an.
// Indice: Pense à qui est propriétaire de la chaîne de caractères.
fn happy_birthday(user: User) -> User {
    // Complète cette fonction
    user
}

fn main() {
    // Tu peux expérimenter ici
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
