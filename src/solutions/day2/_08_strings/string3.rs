pub fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

pub fn first_five(input: &str) -> String {
    if input.chars().count() <= 5 {
        input.to_string()
    } else {
        input.chars().take(5).collect()
    }
}

pub fn concatenate(s1: &str, s2: &str) -> String {
    format!("{} {}", s1, s2)
}

fn main() {
    let test1 = "hello world";
    let test2 = "rust";
    println!("Original: {}", test1);
    println!("En majuscules: {}", to_uppercase(test1));
    println!("5 premiers caractères: {}", first_five(test1));
    println!("Concaténation: {}", concatenate(test1, test2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_uppercase("Rust"), "RUST");
        assert_eq!(to_uppercase(""), "");
    }

    #[test]
    fn test_first_five() {
        assert_eq!(first_five("hello world"), "hello");
        assert_eq!(first_five("rust"), "rust");
        assert_eq!(first_five("hi"), "hi");
        assert_eq!(first_five(""), "");
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate("hello", "world"), "hello world");
        assert_eq!(concatenate("", "rust"), " rust");
        assert_eq!(concatenate("programming", ""), "programming ");
    }
}
