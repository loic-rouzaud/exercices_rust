pub fn parse_number(input: &str) -> Result<i32, String> {
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

fn safe_division(a: f64, b: f64) -> Result<f64, String> {
    match b {
        0.0 => Err(String::from("Division par zero")),
        _ => Ok(a / b),
    }
}
