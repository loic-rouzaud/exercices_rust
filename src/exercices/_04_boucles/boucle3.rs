fn is_prime(n: u32) -> bool {
    // TODO() : Implémenter une fonction qui vérifie si un nombre est premier.
    // Un nombre est premier s'il n'est divisible que par 1 et lui-même.
    // 0 et 1 ne sont pas premiers.
    // Placeholder return value
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(25), false);

        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
    }
}
