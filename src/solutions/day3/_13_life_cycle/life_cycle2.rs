struct TextImportant<'a> {
    part: &'a str,
}

fn first_line<'a>(text: &'a str) -> TextImportant<'a> {
    let first = text.lines().next().unwrap_or("");
    TextImportant { part: first }
}

fn shortest_text<'a>(text1: &'a str, text2: &'a str) -> TextImportant<'a> {
    if text1.len() <= text2.len() {
        TextImportant { part: text1 }
    } else {
        TextImportant { part: text2 }
    }
}

fn get_static_text() -> TextImportant<'static> {
    TextImportant {
        part: "Ceci est un texte avec une durée de vie statique",
    }
}

fn main() {
    let text = "Première ligne\nDeuxième ligne";
    let important = first_line(text);
    println!("Partie importante: {}", important.part);

    let text1 = "Court";
    let text2 = "Plus long";
    let shortest = shortest_text(text1, text2);
    println!("Texte le plus court: {}", shortest.part);

    let static_text = get_static_text();
    println!("Texte statique: {}", static_text.part);
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

        let text3 = String::from("Texte temporaire court");
        let result = shortest_text(text1, &text3);
        assert_eq!(result.part, "Texte court");
    }
}
