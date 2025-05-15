// TODO(): Implémenter les fonctions suivantes pour manipuler des chaînes de caractères.
// Concentrez-vous sur la distinction entre &str et String.

// Cette fonction prend une chaîne &str et retourne une nouvelle String
// qui contient la même chaîne mais en majuscules.
pub fn to_uppercase(input: &str) -> String {
    // Votre code ici
}

// Cette fonction prend une chaîne &str et retourne une nouvelle String
// qui ne contient que les 5 premiers caractères de la chaîne d'entrée.
// Si la chaîne d'entrée a moins de 5 caractères, retourner la chaîne entière.
pub fn first_five(input: &str) -> String {
    // Votre code ici
}

// Cette fonction prend deux chaînes &str et retourne une nouvelle String
// qui est la concaténation des deux chaînes, séparées par un espace.
pub fn concatenate(s1: &str, s2: &str) -> String {
    // Votre code ici
}

fn main() {
    // Vous pouvez tester vos fonctions ici
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
