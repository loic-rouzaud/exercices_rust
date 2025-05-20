fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    println!("{}", square(5));
    println!("{}", 3 * square(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_positive() {
        assert_eq!(square(5), 25);
    }

    #[test]
    fn test_square_zero() {
        assert_eq!(square(0), 0);
    }

    #[test]
    fn test_square_negative() {
        assert_eq!(square(-3), 9);
    }
}
