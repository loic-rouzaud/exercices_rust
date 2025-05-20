// Créez votre fonction ici
todo()!;

fn main() {
    // TODO: Exercice 3 - Fonction avec plusieurs arguments et retour conditionnel
    // 1. Crée une fonction nommée 'is_divisible' qui prend deux paramètres:
    //    - 'number' de type u32
    //    - 'divisor' de type u32
    // 2. Cette fonction doit retourner un booléen (bool):
    //    - true si 'number' est divisible par 'divisor' (number % divisor == 0)
    //    - false sinon
    // 3. Ajoute une vérification pour éviter la division par zéro:
    //    - Si 'divisor' est 0, retourne false
    // 4. Utilise cette fonction pour vérifier si 15 est divisible par 3, 4, et 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_true() {
        assert_eq!(is_divisible(15, 3), true);
        assert_eq!(is_divisible(15, 5), true);
        assert_eq!(is_divisible(12, 4), true);
    }

    #[test]
    fn test_is_divisible_false() {
        assert_eq!(is_divisible(15, 4), false);
        assert_eq!(is_divisible(15, 7), false);
        assert_eq!(is_divisible(17, 3), false);
    }

    #[test]
    fn test_is_divisible_edge_cases() {
        assert_eq!(is_divisible(15, 0), false);
        assert_eq!(is_divisible(15, 1), true);
        assert_eq!(is_divisible(0, 5), true);
        assert_eq!(is_divisible(0, 0), false);
    }
}
