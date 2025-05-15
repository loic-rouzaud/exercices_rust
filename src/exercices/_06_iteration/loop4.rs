pub fn unique_words(sentences: Vec<&str>) -> Vec<String> {
    // TODO() : Implémenter une fonction qui extrait tous les mots uniques
    // d'un ensemble de phrases, les convertit en minuscules, les trie
    // et élimine les doublons
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_words_basic() {
        let sentences = vec!["Hello world", "hello Rust", "World of programming"];
        let result = unique_words(sentences);
        assert_eq!(result, vec!["hello", "of", "programming", "rust", "world"]);
    }

    #[test]
    fn test_unique_words_empty() {
        let sentences: Vec<&str> = vec![];
        let result = unique_words(sentences);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_unique_words_with_punctuation() {
        let sentences = vec!["Hello, world!", "World-class programming."];
        let result = unique_words(sentences);
        assert_eq!(result, vec!["class", "hello", "programming", "world"]);
    }
}
