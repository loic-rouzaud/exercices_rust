fn main() {
    // TODO: Exercice 3
    // 1. Crée une variable 'prenom' de type &str avec ton prénom
    // 2. Crée une variable 'nom' de type &str avec ton nom
    // 3. Crée une variable 'nom_complet' de type String qui contient le prénom et le nom concaténés avec un espace
    // 4. Affiche la longueur de nom_complet et son contenu
}

// Aidez vous des tests pour compléter l'exercice
#[cfg(test)]
mod tests {
    #[test]
    fn test_str_references() {
        let prenom = "Marie";
        let nom = "Durand";

        assert_eq!(prenom, "Marie");
        assert_eq!(nom, "Durand");
    }

    #[test]
    fn test_concatenation() {
        let prenom = "Marie";
        let nom = "Durand";

        let nom_complet = format!("{} {}", prenom, nom);
        assert_eq!(nom_complet, "Marie Durand");
    }

    #[test]
    fn test_string_length() {
        let prenom = "Marie";
        let nom = "Durand";

        let nom_complet = format!("{} {}", prenom, nom);
        assert_eq!(nom_complet.len(), 12);
    }
}
