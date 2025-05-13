fn get_element<T: Clone>(vec: &[T], index: usize) -> Option<T> {
    // TODO() : Implémenter la fonction get_element qui récupère un élément à l'index
    // spécifié et le clone. Retourner Some(élément cloné) si l'index est valide,
    // None sinon. Utiliser la méthode get et cloned.
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_element_valid_index() {
        let vec = vec![10, 20, 30, 40, 50];
        let result = get_element(&vec, 2);
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_get_element_out_of_bounds() {
        let vec = vec![10, 20, 30];
        let result = get_element(&vec, 5);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_element_empty() {
        let vec: Vec<i32> = vec![];
        let result = get_element(&vec, 0);
        assert_eq!(result, None);
    }
}
