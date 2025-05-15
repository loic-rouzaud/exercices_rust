pub fn sum_vector(numbers: &Vec<i32>) -> i32 {
    // TODO() : Implémenter une fonction qui calcule la somme de tous les éléments
    // d'un vecteur d'entiers. Si le vecteur est vide, retourner 0.
    // Utilisez une boucle for (for i in 0..numbers.len()) pour accéder aux éléments
    // du vecteur via leur indice.
}

pub fn double_vector(numbers: &mut Vec<i32>) {
    // TODO() : Implémenter une fonction qui multiplie par 2 chaque élément du vecteur.
    // La fonction doit modifier le vecteur passé en paramètre (ne pas créer un nouveau vecteur).
    // Utilisez une boucle for basée sur les indices pour accéder et modifier les éléments.
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vector_normal() {
        assert_eq!(sum_vector(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vector(&vec![10, 20, 30]), 60);
    }

    #[test]
    fn test_sum_vector_negative() {
        assert_eq!(sum_vector(&vec![-1, -2, -3]), -6);
        assert_eq!(sum_vector(&vec![-5, 5, -10, 10]), 0);
    }

    #[test]
    fn test_sum_vector_empty() {
        assert_eq!(sum_vector(&vec![]), 0);
    }
}
