use std::collections::HashMap;

pub fn count_chars(text: &str) -> HashMap<char, usize> {
    text.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

pub fn word_frequency(text: &str) -> HashMap<String, usize> {
    text.split_whitespace().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(w.to_lowercase().to_string()).or_insert(0) += 1;
        acc
    })
}
