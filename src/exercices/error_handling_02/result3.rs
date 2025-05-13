fn safe_division(a: f64, b: f64) -> Result<f64, String> {
    // TODO() : Implémenter une fonction qui effectue une division sécurisée
    // Retourner Err avec un message approprié si b est égal à zéro
    // Retourner Ok(résultat) si la division est possible
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_division_valid() {
        let result = safe_division(10.0, 2.0);
        assert_eq!(result, Ok(5.0));
    }

    #[test]
    fn test_safe_division_by_zero() {
        let result = safe_division(10.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_division_negative() {
        let result = safe_division(-10.0, 5.0);
        assert_eq!(result, Ok(-2.0));
    }
}
