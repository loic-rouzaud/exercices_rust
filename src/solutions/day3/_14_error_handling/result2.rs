fn parse_number(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Conversion errrors")),
    }
}

fn sum_strings(s1: &str, s2: &str) -> Result<i32, String> {
    let num1 = parse_number(s1)?;
    let num2 = parse_number(s2)?;

    Ok(num1 + num2)
}

fn main() {
    // pour tester
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_valid() {
        let result = parse_number("42");
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_parse_number_negative() {
        let result = parse_number("-123");
        assert_eq!(result, Ok(-123));
    }

    #[test]
    fn test_parse_number_invalid() {
        let result = parse_number("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_strings_valid() {
        let result = sum_strings("10", "20");
        assert_eq!(result, Ok(30));
    }

    #[test]
    fn test_sum_strings_first_invalid() {
        let result = sum_strings("abc", "20");
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_strings_second_invalid() {
        let result = sum_strings("10", "xyz");
        assert!(result.is_err());
    }
}
