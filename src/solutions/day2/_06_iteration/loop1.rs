pub fn sum_even_numbers_like_c(start: i32, end: i32) -> i32 {
    let mut res = 0;
    for i in start..end {
        if i % 2 == 0 {
            res += i;
        }
    }
    res
}

pub fn sum_even_numbers(start: i32, end: i32) -> i32 {
    (start..end).filter(|&x| x % 2 == 0).sum()
}

fn sum_even_numbers_cpy(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&number| number % 2 == 0).sum() // une autre maniere de faire
}
fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_numbers_1() {
        assert_eq!(sum_even_numbers(1, 10), 30);
    }

    #[test]
    fn test_sum_even_numbers_2() {
        assert_eq!(sum_even_numbers(5, 15), 50);
    }

    #[test]
    fn test_sum_even_numbers_3() {
        assert_eq!(sum_even_numbers(-5, 5), 0);
    }

    #[test]
    fn test_sum_even_numbers_cpy_1() {
        assert_eq!(sum_even_numbers_cpy(&[1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn test_sum_even_numbers_cpy_2() {
        assert_eq!(sum_even_numbers_cpy(&[-2, -1, 0, 1, 2]), 0);
    }

    #[test]
    fn test_sum_even_numbers_cpy_3() {
        assert_eq!(sum_even_numbers_cpy(&[10, 11, 12, 13, 14]), 36);
    }
}
