// TODO(): Définissez une structure générique Pair qui peut contenir deux valeurs de même type.
// La structure doit avoir deux champs: first et second, tous deux du même type générique.

// TODO(): Implémentez les méthodes suivantes pour la structure Pair:
// 1. new - un constructeur qui crée une nouvelle paire
// 2. swap - une méthode qui échange les valeurs first et second
// 3. into_tuple - une méthode qui convertit la paire en tuple

impl<T> Pair<T> {
    // Constructeur: crée une nouvelle paire avec les valeurs données
    fn new(/* complétez les paramètres */) -> Self {
        // Complétez l'implémentation
    }

    // Échange les valeurs first et second
    fn swap(&mut self) {
        // Complétez l'implémentation
    }

    // Convertit la paire en tuple (T, T)
    // Note: cette méthode consomme self (prend ownership)
    fn into_tuple(self) -> (T, T) {
        // Complétez l'implémentation
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
