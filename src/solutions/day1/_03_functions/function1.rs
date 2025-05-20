fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("Claude"));
    println!("{}", greet("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greet_test(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    #[test]
    fn test_greet_name() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_greet_world() {
        assert_eq!(greet("world"), "Hello, world!");
    }

    #[test]
    fn test_greet_empty() {
        // A function should also work with an empty string
        assert_eq!(greet(""), "Hello, !");
    }
}
