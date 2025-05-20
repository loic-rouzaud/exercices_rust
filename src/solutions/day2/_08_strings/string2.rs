fn is_numeric(arg: &str) -> bool {
    arg.chars().all(|c| c.is_digit(10))
}

fn count_chars(arg: String) -> usize {
    arg.chars().count()
}

fn main() {
    let result1 = is_numeric("12345");
    let result2 = count_chars("hello".to_string());
    let result3 = count_chars(String::from("67890"));
    let result4 = count_chars("rust is great!".to_owned());
    let result5 = count_chars("42".into());
    let result6 = count_chars(format!("Number: {}", 7));
    let result7 = is_numeric(&String::from("Hello World")[0..5]);
    let result8 = is_numeric("  abc123  ".trim());
    let result9 = count_chars("Programming is fun".replace("fun", "awesome"));
    let result10 = count_chars("TEST".to_lowercase());

    println!(
        "Results: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        result1, result2, result3, result4, result5, result6, result7, result8, result9, result10
    );
}
