pub fn classify_number(n: i32) -> String {
    match (n > 0, n < 0, n % 2 == 0) {
        (true, false, true) => String::from("Positif et pair"),
        (true, false, false) => String::from("Positif et impair"),
        (false, true, true) => String::from("Négatif et pair"),
        (false, true, false) => String::from("Négatif et impair"),
        _ => String::from("Zéro"),
    }
}

fn main() {
    println!("2: {}", classify_number(2));
    println!("3: {}", classify_number(3));
    println!("-4: {}", classify_number(-4));
    println!("-5: {}", classify_number(-5));
    println!("0: {}", classify_number(0));
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
