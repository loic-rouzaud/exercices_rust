fn is_divisible(number: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    number % divisor == 0
}

fn main() {
    println!("15 est divisible par 3: {}", is_divisible(15, 3));
    println!("15 est divisible par 4: {}", is_divisible(15, 4));
    println!("15 est divisible par 5: {}", is_divisible(15, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_divisible_true() {
        assert_eq!(is_divisible(15, 3), true);
        assert_eq!(is_divisible(15, 5), true);
        assert_eq!(is_divisible(12, 4), true);
    }

    #[test]
    fn test_is_divisible_false() {
        assert_eq!(is_divisible(15, 4), false);
        assert_eq!(is_divisible(15, 7), false);
        assert_eq!(is_divisible(17, 3), false);
    }

    #[test]
    fn test_is_divisible_edge_cases() {
        assert_eq!(is_divisible(15, 0), false);
        assert_eq!(is_divisible(15, 1), true);
        assert_eq!(is_divisible(0, 5), true);
        assert_eq!(is_divisible(0, 0), false);
    }
}
