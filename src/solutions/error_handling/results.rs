#[derive(Debug)]
#[allow(dead_code)]
enum ParseError {
    EmptyInput,
    InvalidNumber,
    OutOfRange,
}

#[allow(dead_code)]
fn parse_age(input: &str) -> Result<u8, ParseError> {
    // Parser une chaîne en âge (0-120)
    // Gérer les cas : chaîne vide, non-numérique, hors limites
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

#[allow(dead_code)]
pub fn parse_number(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Conversion errrors")),
    }
}

#[allow(dead_code)]
fn sum_strings(s1: &str, s2: &str) -> Result<i32, String> {
    let num1 = parse_number(s1)?;
    let num2 = parse_number(s2)?;

    Ok(num1 + num2)
}

#[allow(dead_code)]
fn safe_division(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0 => Err(String::from("Division par zero")),
        _ => Ok(a / b),
    }
}
