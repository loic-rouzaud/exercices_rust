pub fn classify_number(n: i32) -> String {
    // TODO(): Implémenter une fonction qui catégorise un nombre selon ces règles:
    // - Si le nombre est positif ET pair: retourner "Positif et pair"
    // - Si le nombre est positif ET impair: retourner "Positif et impair"
    // - Si le nombre est négatif ET pair: retourner "Négatif et pair"
    // - Si le nombre est négatif ET impair: retourner "Négatif et impair"
    // - Si le nombre est zéro: retourner "Zéro"
    //
    // Indice: vous pouvez utiliser les tuples et le pattern matching pour combiner
    // plusieurs conditions.
}

fn main() {
    // pour tester votre fonction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_even() {
        assert_eq!(classify_number(2), "Positif et pair");
        assert_eq!(classify_number(10), "Positif et pair");
        assert_eq!(classify_number(100), "Positif et pair");
    }

    #[test]
    fn test_positive_odd() {
        assert_eq!(classify_number(1), "Positif et impair");
        assert_eq!(classify_number(7), "Positif et impair");
        assert_eq!(classify_number(99), "Positif et impair");
    }

    #[test]
    fn test_negative_even() {
        assert_eq!(classify_number(-2), "Négatif et pair");
        assert_eq!(classify_number(-10), "Négatif et pair");
        assert_eq!(classify_number(-100), "Négatif et pair");
    }

    #[test]
    fn test_negative_odd() {
        assert_eq!(classify_number(-1), "Négatif et impair");
        assert_eq!(classify_number(-7), "Négatif et impair");
        assert_eq!(classify_number(-99), "Négatif et impair");
    }

    #[test]
    fn test_zero() {
        assert_eq!(classify_number(0), "Zéro");
    }
}
