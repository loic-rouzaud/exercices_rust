fn main() {
    // TODO: Exercice 2
    // 1. Déclare une variable mutable 'compteur' avec la valeur initiale 0
    // 2. Incrémente la valeur de 'compteur' de 5
    // 3. Utilise le shadowing pour transformer 'compteur' en f32 et divise sa valeur par 2
    // 4. Affiche la valeur finale
}

// Aidez vous des tests pour compléter l'exercice
#[cfg(test)]
mod tests {
    #[test]
    fn test_mutabilite() {
        let mut compteur = 0;
        compteur = compteur + 5;
        assert_eq!(compteur, 5);
    }

    #[test]
    fn test_shadowing() {
        let mut compteur = 0;
        compteur = compteur + 5;
        let compteur = compteur as f32 / 2.0;
        assert_eq!(compteur, 2.5);
    }

    #[test]
    fn test_changement_type() {
        let nombre = 10;
        let nombre = nombre.to_string();
        // Vérifie que le type a changé en comparant avec une chaîne
        assert_eq!(nombre, "10");
    }
}
