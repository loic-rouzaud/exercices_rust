fn sum_vector(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in 0..numbers.len() {
        sum += numbers[i];
    }

    return sum;
}

fn double_vector(numbers: &mut Vec<i32>) {
    for i in 0..numbers.len() {
        numbers[i] *= 2;
    }
}

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    println!("Sum: {}", sum_vector(&vec));

    double_vector(&mut vec);
    println!("After doubling: {:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_vector_normal() {
        assert_eq!(sum_vector(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vector(&vec![10, 20, 30]), 60);
    }

    #[test]
    fn test_sum_vector_negative() {
        assert_eq!(sum_vector(&vec![-1, -2, -3]), -6);
        assert_eq!(sum_vector(&vec![-5, 5, -10, 10]), 0);
    }

    #[test]
    fn test_sum_vector_empty() {
        assert_eq!(sum_vector(&vec![]), 0);
    }
}
