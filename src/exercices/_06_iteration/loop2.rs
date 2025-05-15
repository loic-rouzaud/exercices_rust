pub fn process_strings(strings: Vec<&str>, min_lenght: usize) -> usize {
    // TODO() : Implémenter une fonction qui prend un vecteur de chaînes,
    // calcule la longueur de chaque chaîne, filtre celles qui dépassent
    // une longueur minimale, puis retourne la somme des longueurs restantes
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_strings_basic() {
        let strings = vec!["hello", "world", "rust", "programming"];
        assert_eq!(process_strings(strings, 5), 15);
    }

    #[test]
    fn test_process_strings_empty() {
        let strings: Vec<&str> = vec![];
        assert_eq!(process_strings(strings, 3), 0);
    }

    #[test]
    fn test_process_strings_none_match() {
        let strings = vec!["a", "bc", "def"];
        assert_eq!(process_strings(strings, 4), 0);
    }
}
