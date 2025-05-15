#[derive(Debug, PartialEq)]
enum ParseError {
    EmptyInput,
    InvalidNumber,
    OutOfRange,
}

fn parse_age(input: &str) -> Result<u8, ParseError> {
    // TODO() : Implémenter une fonction qui analyse une chaîne de caractères
    // et la convertit en âge (u8). La fonction doit :
    // - Retourner ParseError::EmptyInput si la chaîne est vide
    // - Retourner ParseError::InvalidNumber si la chaîne n'est pas un nombre valide
    // - Retourner ParseError::OutOfRange si le nombre est supérieur à 120
    // - Retourner Ok(age) si l'âge est valide (entre 0 et 120)
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_age_valid() {
        let result = parse_age("25");
        assert_eq!(result, Ok(25));
    }

    #[test]
    fn test_parse_age_empty() {
        let result = parse_age("");
        assert!(matches!(result, Err(ParseError::EmptyInput)));
    }

    #[test]
    fn test_parse_age_out_of_range() {
        let result = parse_age("150");
        assert!(matches!(result, Err(ParseError::OutOfRange)));
    }
}
