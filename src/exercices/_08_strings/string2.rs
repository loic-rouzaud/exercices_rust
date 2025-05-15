fn is_numeric(arg: &str) -> bool {
    // Returns true if the string contains only numeric characters
    arg.chars().all(|c| c.is_digit(10))
}

fn count_chars(arg: String) -> usize {
    // Returns the number of characters in the string
    arg.chars().count()
}

// TODO(): Voici une liste d'expressions qui produisent soit un `String`, soit un `&str`.
// Votre tâche est de remplacer les appels `placeholder(...)` par des appels à
// `is_numeric(...)` ou `count_chars(...)` selon le type que vous pensez que
// chaque expression produit.
//
// Indice: utilisez le pattern matching pour comprendre quel type est retourné par
// chaque expression.
fn main() {
    let result1 = placeholder("12345");
    let result2 = placeholder("hello".to_string());
    let result3 = placeholder(String::from("67890"));
    let result4 = placeholder("rust is great!".to_owned());
    let result5 = placeholder("42".into());
    let result6 = placeholder(format!("Number: {}", 7));
    let result7 = placeholder(&String::from("Hello World")[0..5]);
    let result8 = placeholder("  abc123  ".trim());
    let result9 = placeholder("Programming is fun".replace("fun", "awesome"));
    let result10 = placeholder("TEST".to_lowercase());

    println!(
        "Results: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        result1, result2, result3, result4, result5, result6, result7, result8, result9, result10
    );
}
