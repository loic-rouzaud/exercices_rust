#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidNumber,
    OutOfRange,
}

fn parse_age(input: &str) -> Result<u8, ParseError> {
    // TODO() : Implémenter une fonction qui analyse une chaîne de caractères
    // et la convertit en âge (u8). La fonction doit :
    // - Retourner ParseError::EmptyInput si la chaîne est vide
    // - Retourner ParseError::InvalidNumber si la chaîne n'est pas un nombre valide
    // - Retourner ParseError::OutOfRange si le nombre est supérieur à 120
    // - Retourner Ok(age) si l'âge est valide (entre 0 et 120)
}

pub fn parse_number(input: &str) -> Result<i32, String> {
    // TODO() : Implémenter une fonction qui convertit une chaîne en nombre entier
    // Retourner Ok(nombre) si la conversion réussit
    // Retourner Err avec un message d'erreur si la conversion échoue
}

fn sum_strings(s1: &str, s2: &str) -> Result<i32, String> {
    // TODO() : Implémenter une fonction qui additionne deux nombres représentés
    // comme des chaînes de caractères. Utiliser parse_number et l'opérateur ? pour
    // propager les erreurs. Retourner Ok(somme) si tout se passe bien.
}

fn safe_division(a: f64, b: f64) -> Result<f64, String> {
    // TODO() : Implémenter une fonction qui effectue une division sécurisée
    // Retourner Err avec un message approprié si b est égal à zéro
    // Retourner Ok(résultat) si la division est possible
}
