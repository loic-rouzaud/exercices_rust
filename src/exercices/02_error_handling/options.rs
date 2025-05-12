fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    // TODO() : Implémenter la fonction find_first qui cherche le premier élément
    // dans le tableau qui satisfait le prédicat donné. Retourner Some(&element)
    // si trouvé, None sinon. Utiliser l'itérateur et la méthode find.
}

fn get_element<T: Clone>(vec: &[T], index: usize) -> Option<T> {
    // TODO() : Implémenter la fonction get_element qui récupère un élément à l'index
    // spécifié et le clone. Retourner Some(élément cloné) si l'index est valide,
    // None sinon. Utiliser la méthode get et cloned.
}

fn process_words(words: Vec<&str>) -> Vec<String> {
    // TODO() : Implémenter la fonction process_words qui filtre les mots
    // ayant plus de 3 caractères, les convertit en majuscules et les
    // collecte dans un nouveau vecteur. Utiliser les méthodes filter, map, collect.
}
