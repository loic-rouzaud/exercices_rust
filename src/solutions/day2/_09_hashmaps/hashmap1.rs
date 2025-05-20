use std::collections::HashMap;

fn count_chars(text: &str) -> HashMap<char, usize> {
    text.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
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
    fn test_count_chars_basic() {
        let text = "hello";
        let counts = count_chars(text);

        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 2);
        expected.insert('o', 1);

        assert_eq!(counts, expected);
    }

    #[test]
    fn test_count_chars_empty() {
        let text = "";
        let counts = count_chars(text);

        let expected: HashMap<char, usize> = HashMap::new();
        assert_eq!(counts, expected);
    }

    #[test]
    fn test_count_chars_with_spaces() {
        let text = "hello world";
        let counts = count_chars(text);

        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 3);
        expected.insert('o', 2);
        expected.insert(' ', 1);
        expected.insert('w', 1);
        expected.insert('r', 1);
        expected.insert('d', 1);

        assert_eq!(counts, expected);
    }
}
