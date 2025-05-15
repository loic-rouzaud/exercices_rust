fn main() {
    // TODO: Exercice 1
    // 1. Déclare une variable 'age' de type i32 avec la valeur 25
    // 2. Déclare une variable 'taille' de type f32 avec la valeur 1.75
    // 3. Affiche ces variables avec println!
}

// Aidez vous des tests pour compléter l'exercice
#[cfg(test)]
mod tests {
    #[test]
    fn test_declaration_i32() {
        let age: i32 = 25;
        assert_eq!(age, 25);
    }

    #[test]
    fn test_declaration_f32() {
        let taille: f32 = 1.75;
        assert_eq!(taille, 1.75);
    }

    #[test]
    fn test_types_differents() {
        let age: i32 = 25;
        let taille: f32 = 1.75;

        assert_eq!(age as f32, 25.0);
        assert_ne!(age, taille as i32);
    }
}
