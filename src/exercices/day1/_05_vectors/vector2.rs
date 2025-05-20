pub fn filter_even_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    // TODO() : Implémenter une fonction qui crée et retourne un nouveau vecteur
    // contenant uniquement les nombres pairs du vecteur d'entrée.
    // Utilisez une boucle for basée sur les indices pour parcourir le vecteur d'entrée,
    // et la méthode ??? pour ajouter les nombres pairs au nouveau vecteur.
    Vec::new() // Placeholder return value
}

pub fn merge_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    // TODO() : Implémenter une fonction qui fusionne deux vecteurs en alternant leurs éléments.
    // Par exemple, si vec1 = [1, 3, 5] et vec2 = [2, 4, 6], le résultat doit être [1, 2, 3, 4, 5, 6].
    // Si l'un des vecteurs est plus long que l'autre, ajouter les éléments restants à la fin.
    // Utilisez des boucles while ou for pour parcourir les vecteurs et construire le résultat.
    Vec::new() // Placeholder return value
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_even_numbers_normal() {
        assert_eq!(filter_even_numbers(&vec![1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(filter_even_numbers(&vec![10, 15, 20, 25]), vec![10, 20]);
    }

    #[test]
    fn test_filter_even_numbers_negative() {
        assert_eq!(filter_even_numbers(&vec![-2, -1, 0, 1, 2]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_filter_even_numbers_empty() {
        assert_eq!(filter_even_numbers(&vec![]), Vec::<i32>::new());
        assert_eq!(filter_even_numbers(&vec![1, 3, 5]), Vec::<i32>::new());
    }
}
