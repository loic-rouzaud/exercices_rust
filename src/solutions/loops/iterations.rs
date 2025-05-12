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

#[allow(dead_code)]
fn sum_even_numbers1(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&number| number % 2 == 0).sum() // une autre maniere de faire
}

#[allow(dead_code)]
pub fn process_strings(strings: Vec<&str>, min_lenght: usize) -> usize {
    strings
        .iter()
        .map(|string| string.len())
        .filter(|&len| len > min_lenght)
        .sum()
}

pub fn square_roots_of_even(numbers: Vec<i32>) -> Vec<f64> {
    numbers
        .iter()
        .filter(|&number| number % 2 == 0)
        .map(|&number| (number as f64).sqrt())
        .collect()
}

pub fn unique_words(sentences: Vec<&str>) -> Vec<String> {
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

pub fn to_uppercase(strings: &[String]) -> Vec<String> {
    strings.iter().map(|string| string.to_uppercase()).collect()
}
