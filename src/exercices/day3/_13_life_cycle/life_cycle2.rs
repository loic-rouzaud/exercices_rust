// TODO: Ajoute les annotations de durée de vie manquantes sur la structure
// et les fonctions ci-dessous.
struct TextImportant {
    part: &str,
}

// TODO: Ajoute les annotations de durée de vie nécessaires sur cette fonction.
// La fonction doit retourner une instance de TextImportant qui référence
// la première ligne du texte fourni.
fn first_line(text: &str) -> TextImportant {
    let first = text.lines().next().unwrap_or("");
    TextImportant { part: first }
}

// TODO: Ajoute les annotations de durée de vie correctes sur cette fonction.
// Elle prend deux textes avec potentiellement des durées de vie différentes
// et retourne une structure TextImportant qui référence le plus court des deux.
fn shortest_text(text1: &str, text2: &str) -> TextImportant {
    if text1.len() <= text2.len() {
        TextImportant { part: text1 }
    } else {
        TextImportant { part: text2 }
    }
}

// TODO: Implémente cette fonction qui utilise une référence statique.
// Elle doit retourner une instance de TextImportant avec une référence
// à une chaîne statique (durée de vie 'static).
fn get_static_text() -> TextImportant {
    // Utilise une chaîne littérale (qui a une durée de vie 'static)
    // pour créer et retourner une instance de TextImportant
    unimplemented!()
}

fn main() {
    // Tu peux expérimenter ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_line() {
        let text = "Première ligne.\nDeuxième ligne.\nTroisième ligne.";
        let important = first_line(text);

        assert_eq!(important.part, "Première ligne.");
    }

    #[test]
    fn test_shortest_text() {
        let text1 = "Texte court";
        let text2 = "Texte beaucoup plus long";

        let important = shortest_text(text1, text2);
        assert_eq!(important.part, "Texte court");

        let result;
        {
            let temp_text = String::from("Texte temporaire court");
            result = shortest_text(text1, &temp_text);
        }
        assert_eq!(result.part, "Texte court");
    }
}
