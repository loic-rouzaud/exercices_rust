fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32 + 1;

    for i in 3..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    println!("2 is prime: {}", is_prime(2));
    println!("4 is prime: {}", is_prime(4));
    println!("17 is prime: {}", is_prime(17));
    println!("25 is prime: {}", is_prime(25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(15), false);
        assert_eq!(is_prime(25), false);

        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
    }
}
