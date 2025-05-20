use std::collections::HashMap;

pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    text.split_whitespace().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(w.to_lowercase().to_string()).or_insert(0) += 1;
        acc
    })
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency_basic() {
        let text = "hello world hello";
        let counts = word_frequency(text);

        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), 2);
        expected.insert("world".to_string(), 1);

        assert_eq!(counts, expected);
    }

    #[test]
    fn test_word_frequency_empty() {
        let text = "";
        let counts = word_frequency(text);

        let expected: HashMap<String, usize> = HashMap::new();
        assert_eq!(counts, expected);
    }

    #[test]
    fn test_word_frequency_case_insensitive() {
        let text = "Hello World hello WORLD";
        let counts = word_frequency(text);

        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), 2);
        expected.insert("world".to_string(), 2);

        assert_eq!(counts, expected);
    }
}
