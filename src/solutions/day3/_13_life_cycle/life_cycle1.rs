fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("longue chaîne");
    let string2 = String::from("plus courte");

    let result = longest_string(&string1, &string2);
    println!("La chaîne la plus longue est: {}", result);
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
