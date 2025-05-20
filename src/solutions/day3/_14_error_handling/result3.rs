fn safe_division(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0 => Err(String::from("Division par zero")),
        _ => Ok(a / b),
    }
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
