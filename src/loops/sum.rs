use std::collections::HashSet;

pub fn sum_even_numbers_like_c(start: i32, end: i32) -> i32 {
    let mut res = 0;
    for i in start..end {
        if i % 2 == 0 {
            res += i;
        }
    }
    res
}

pub fn sum_even_numbers_like_rust(start: i32, end: i32) -> i32 {
    (start..end).filter(|&x| x % 2 == 0).sum()
}

pub fn process_strings(strings: Vec<&str>, min_lenght: usize) -> usize {
    strings
        .iter()
        .map(|string| string.len())
        .filter(|&len| len > min_lenght)
        .sum()
}

fn square_roots_of_even(numbers: Vec<i32>) -> Vec<f64> {
    numbers
        .iter()
        .filter(|&number| number % 2 == 0)
        .map(|&number| (number as f64).sqrt())
        .collect()
}

// fn unique_words(sentences: Vec<&str>) -> Vec<String> {
//     let mut words: Vec<String> = sentences
//         .iter()
//         .flat_map(|&sentence| sentence.to_lowercase().split_whitespace())
//         .map(|word| word.to_string())
//         .collect();
//     words.
// }
