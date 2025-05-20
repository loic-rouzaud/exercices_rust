fn unique_words(sentences: Vec<&str>) -> Vec<String> {
    let mut words: Vec<String> = sentences
        .into_iter()
        .flat_map(|sentence| {
            let lower = sentence.to_lowercase();
            lower
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .collect();

    words.sort(); // obligatoire pour que dedup() fonctionne
    words.dedup();
    words
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
