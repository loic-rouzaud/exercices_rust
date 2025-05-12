#[allow(dead_code)]
fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|item| predicate(item)) // find renvoie deja Some(..) ou None
}

#[allow(dead_code)]
fn get_element<T: Clone>(vec: &[T], index: usize) -> Option<T> {
    vec.get(index).cloned()
}

#[allow(dead_code)]
fn process_words(words: Vec<&str>) -> Vec<String> {
    words
        .iter()
        .filter(|word| word.len() > 3)
        .map(|word| word.to_uppercase())
        .collect()
}
