fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    // TODO() : Implémenter la fonction find_first qui cherche le premier élément
    // dans le tableau qui satisfait le prédicat donné. Retourner Some(&element)
    // si trouvé, None sinon. Utiliser l'itérateur et la méthode find.
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_exists() {
        let numbers = [1, 2, 3, 4, 5];
        let result = find_first(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Some(&2));
    }

    #[test]
    fn test_find_first_not_found() {
        let numbers = [1, 3, 5, 7, 9];
        let result = find_first(&numbers, |x| x % 2 == 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_first_empty() {
        let numbers: [i32; 0] = [];
        let result = find_first(&numbers, |x| x % 2 == 0);
        assert_eq!(result, None);
    }
}
