// TODO: Corrige l'erreur de compilation en ajoutant les annotations de durée de vie appropriées.
// Indice: La fonction renvoie une référence qui doit avoir une durée de vie explicite.
fn longest_string(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    // Tu peux expérimenter ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_string() {
        let string1 = String::from("long string is long");
        let string2 = String::from("short");

        let result = longest_string(&string1, &string2);
        assert_eq!(result, "long string is long");

        // TODO: Essaie d'utiliser longest_string avec une référence à une variable de portée limitée.
        // Que se passe-t-il et pourquoi?
        /*
        let result;
        {
            let string3 = String::from("another string");
            result = longest_string(&string1, &string3);
        }
        println!("Le plus long est: {}", result);
        */
    }
}
