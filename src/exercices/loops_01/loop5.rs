pub fn to_uppercase(strings: &[String]) -> Vec<String> {
    // TODO() : Implémenter une fonction qui convertit toutes les chaînes
    // d'un tableau en majuscules et retourne un nouveau vecteur
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_basic() {
        let strings = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
        ];
        let result = to_uppercase(&strings);
        assert_eq!(result, vec!["HELLO", "WORLD", "RUST"]);
    }

    #[test]
    fn test_to_uppercase_empty() {
        let strings: Vec<String> = vec![];
        let result = to_uppercase(&strings);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_to_uppercase_mixed_case() {
        let strings = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("Rust"),
        ];
        let result = to_uppercase(&strings);
        assert_eq!(result, vec!["HELLO", "WORLD", "RUST"]);
    }
}
