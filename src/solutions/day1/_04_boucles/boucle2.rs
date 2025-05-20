fn sum_until(limit: i32) -> i32 {
    if limit < 0 {
        return 0;
    }

    let mut sum = 0;
    let mut current = 0;

    while sum <= limit {
        current += 1;
        sum += current;
    }

    return current;
}

fn factorial(n: u32) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut result: u64 = 1;

    for i in 1..=n {
        result *= i as u64;
    }

    return result;
}

fn main() {
    println!("Sum until 10: {}", sum_until(10));
    println!("Factorial of 5: {}", factorial(5));
}
