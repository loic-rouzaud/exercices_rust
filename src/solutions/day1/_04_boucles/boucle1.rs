fn print_numbers(n: i32) {
    if n <= 0 {
        return;
    }

    for i in 1..=n {
        println!("Number: {}", i);
    }
}

fn main() {
    print_numbers(5);
}
