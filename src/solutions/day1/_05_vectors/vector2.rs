pub fn filter_even_numbers(numbers: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            result.push(numbers[i]);
        }
    }

    return result;
}

pub fn merge_vectors(vec1: &Vec<i32>, vec2: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < vec1.len() || j < vec2.len() {
        if i < vec1.len() {
            result.push(vec1[i]);
            i += 1;
        }

        if j < vec2.len() {
            result.push(vec2[j]);
            j += 1;
        }
    }

    return result;
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter_even_numbers(&numbers);
    println!("Even numbers: {:?}", even_numbers);

    let vec1 = vec![1, 3, 5];
    let vec2 = vec![2, 4, 6, 8, 10];
    let merged = merge_vectors(&vec1, &vec2);
    println!("Merged vectors: {:?}", merged);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_even_numbers_normal() {
        assert_eq!(filter_even_numbers(&vec![1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(filter_even_numbers(&vec![10, 15, 20, 25]), vec![10, 20]);
    }

    #[test]
    fn test_filter_even_numbers_negative() {
        assert_eq!(filter_even_numbers(&vec![-2, -1, 0, 1, 2]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_filter_even_numbers_empty() {
        assert_eq!(filter_even_numbers(&vec![]), Vec::<i32>::new());
        assert_eq!(filter_even_numbers(&vec![1, 3, 5]), Vec::<i32>::new());
    }
}
