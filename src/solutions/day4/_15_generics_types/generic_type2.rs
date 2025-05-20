struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn swap(&mut self) {
        std::mem::swap(&mut self.first, &mut self.second);
    }

    fn into_tuple(self) -> (T, T) {
        (self.first, self.second)
    }
}

fn main() {
    let mut numbers = Pair::new(5, 10);
    println!("Paire initiale: ({}, {})", numbers.first, numbers.second);

    numbers.swap();
    println!("Après échange: ({}, {})", numbers.first, numbers.second);

    let tuple = numbers.into_tuple();
    println!("Tuple: ({}, {})", tuple.0, tuple.1);

    let words = Pair::new(String::from("Hello"), String::from("Rust"));
    println!("Paire de chaînes: ({}, {})", words.first, words.second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_constructor() {
        let p = Pair::new(1, 2);
        assert_eq!(p.first, 1);
        assert_eq!(p.second, 2);

        let s = Pair::new(String::from("a"), String::from("b"));
        assert_eq!(s.first, "a");
        assert_eq!(s.second, "b");
    }

    #[test]
    fn test_pair_swap() {
        let mut p = Pair::new(1, 2);
        p.swap();
        assert_eq!(p.first, 2);
        assert_eq!(p.second, 1);
    }
}
