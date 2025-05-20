#[derive(Debug, PartialEq)]
enum ParseError {
    EmptyInput,
    InvalidNumber,
    OutOfRange,
}

fn parse_age(input: &str) -> Result<u8, ParseError> {
    if input.trim().is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let parsed: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(ParseError::InvalidNumber),
    };

    if parsed <= 120 {
        Ok(parsed)
    } else {
        Err(ParseError::OutOfRange)
    }
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
