// Créez votre fonction ici
todo()!;

fn main() {
    // TODO: Exercice 2 - Fonction avec valeur de retour
    // 1. Crée une fonction nommée 'square' qui prend un paramètre 'x' de type i32
    // 2. Cette fonction doit retourner le carré de x (x * x)
    // 3. Appelle cette fonction avec 5 et affiche le résultat
    // 4. Utilise le résultat de cette fonction dans un calcul : 3 * square(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_positive() {
        assert_eq!(square(5), 25);
    }

    #[test]
    fn test_square_zero() {
        assert_eq!(square(0), 0);
    }

    #[test]
    fn test_square_negative() {
        assert_eq!(square(-3), 9);
    }
}
