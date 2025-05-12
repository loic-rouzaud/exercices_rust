pub fn apply_operations<F>(data: &mut Vec<i32>, mut operations: Vec<F>)
where
    F: FnMut(&mut Vec<i32>),
{
    // TODO() : Implémenter la fonction apply_operations qui applique une série
    // d'opérations (closures) à un vecteur de données.
    // Chaque opération doit être appelée avec le vecteur de données comme argument,
    // permettant à ces opérations de modifier le vecteur.
}
