fn square_roots_of_even(numbers: Vec<i32>) -> Vec<f64> {
    numbers
        .iter()
        .filter(|&number| number % 2 == 0)
        .map(|&number| (number as f64).sqrt())
        .collect()
}
fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_roots_of_even_basic() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result = square_roots_of_even(numbers);
        assert_eq!(result, vec![1.4142135623730951, 2.0, 2.449489742783178]);
    }

    #[test]
    fn test_square_roots_of_even_empty() {
        let numbers: Vec<i32> = vec![];
        let result = square_roots_of_even(numbers);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_square_roots_of_even_no_even() {
        let numbers = vec![1, 3, 5, 7, 9];
        let result = square_roots_of_even(numbers);
        assert_eq!(result, Vec::<f64>::new());
    }
}
