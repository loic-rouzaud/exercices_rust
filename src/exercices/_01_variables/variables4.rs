fn main() {
    // TODO: Exercice 4
    // 1. Déclare une variable 'nombre' de type i32 avec la valeur 42
    // 2. Convertis cette valeur en f32 et stocke-la dans 'nombre_f32'
    // 3. Crée une chaîne 'texte_nombre' qui contient la valeur de 'nombre' convertie en chaîne
    // 4. Affiche les trois variables et leurs types
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_conversion_i32_to_f32() {
        let nombre: i32 = 42;
        let nombre_f32: f32 = nombre as f32;
        assert_eq!(nombre_f32, 42.0);
    }

    #[test]
    fn test_conversion_i32_to_string() {
        let nombre: i32 = 42;
        let texte_nombre = nombre.to_string();
        assert_eq!(texte_nombre, "42");
    }

    #[test]
    fn test_types_differents() {
        let nombre: i32 = 42;
        let nombre_f32: f32 = nombre as f32;
        let texte_nombre = nombre.to_string();

        assert_eq!(nombre_f32 as i32, nombre);
        assert_eq!(texte_nombre.parse::<i32>().unwrap(), nombre);
    }
}
