pub fn apply_operations<F>(data: &mut Vec<i32>, mut operations: Vec<F>)
where
    F: FnMut(&mut Vec<i32>),
{
    // TODO() : Implémenter la fonction apply_operations qui applique une série
    // d'opérations (closures) à un vecteur de données.
    // Chaque opération doit être appelée avec le vecteur de données comme argument,
    // permettant à ces opérations de modifier le vecteur.
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_operations_empty_data() {
        let mut data: Vec<i32> = vec![];
        let operations = vec![|v: &mut Vec<i32>| v.push(1), |v: &mut Vec<i32>| v.push(2)];

        apply_operations(&mut data, operations);
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test_apply_operations_empty_ops() {
        let mut data = vec![1, 2, 3];
        let operations: Vec<fn(&mut Vec<i32>)> = vec![];

        apply_operations(&mut data, operations);
        assert_eq!(data, vec![1, 2, 3]);
    }
}
