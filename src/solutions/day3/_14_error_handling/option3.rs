fn process_words(words: Vec<&str>) -> Vec<String> {
    // TODO() : Implémenter la fonction process_words qui filtre les mots
    // ayant plus de 3 caractères, les convertit en majuscules et les
    // collecte dans un nouveau vecteur. Utiliser les méthodes filter, map, collect.
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_words_basic() {
        let words = vec!["the", "quick", "brown", "fox", "jumped"];
        let result = process_words(words);
        assert_eq!(result, vec!["QUICK", "BROWN", "JUMPED"]);
    }

    #[test]
    fn test_process_words_empty() {
        let words: Vec<&str> = vec![];
        let result = process_words(words);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_process_words_no_matching() {
        let words = vec!["a", "an", "the", "in"];
        let result = process_words(words);
        assert_eq!(result, Vec::<String>::new());
    }
}
