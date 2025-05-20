// Créez votre fonction ici
todo()!;

fn main() {
    // TODO: Exercice 1 - Création et appel d'une fonction simple
    // 1. Crée une fonction nommée 'greet' qui prend un paramètre 'name' de type &str
    // 2. Cette fonction doit afficher "Hello, {name}!"
    // 3. Appelle cette fonction avec ton prénom
    // 4. Appelle cette fonction avec "world"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greet_test(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[test]
    fn test_greet_name() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_greet_world() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn test_greet_empty() {
        // A function should also work with an empty string
        assert_eq!(greet(""), "Hello, !");
    }
}
