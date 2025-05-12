//
// Exercice 1
//
// Objectif : Comprendre comment les pointeurs intelligents Box permettent
// des structures de données récursives en Rust en examinant une implémentation
// d'arbre binaire qui organise les valeurs selon leurs propriétés.
//

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub fn is_even(value: i32) -> bool {
    // TODO() : Implémenter une fonction qui détermine si un nombre est pair
    todo!();
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        // TODO() : Créer un nouveau nœud avec la valeur donnée et des branches gauche et droite vides
        todo!();
    }

    pub fn traverse_left_side(&self, visit: &mut dyn FnMut(&T)) {
        // TODO() : Implémenter un parcours du côté gauche de l'arbre qui visite récursivement
        // les nœuds enfants avant d'appliquer la fonction visit sur la valeur actuelle
        todo!();
    }

    pub fn traverse_right_side(&self, visit: &mut dyn FnMut(&T)) {
        // TODO() : Implémenter un parcours du côté droit de l'arbre qui visite récursivement
        // les nœuds enfants avant d'appliquer la fonction visit sur la valeur actuelle
        todo!();
    }
}

impl Node<i32> {
    pub fn insert(&mut self, value: i32) {
        // TODO() : Implémenter l'insertion d'une valeur dans l'arbre
        // Les nombres pairs doivent aller dans le sous-arbre gauche
        // Les nombres impairs doivent aller dans le sous-arbre droit
        // Utiliser la fonction is_even pour déterminer où insérer la valeur
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1_node_creation() {
        let node = Node::new(10);
        assert!(node.left.is_none());
        assert!(node.right.is_none());
        assert_eq!(node.value, 10);
    }
    #[test]
    fn ex1_insertion() {
        let mut root = Node::new(0);
        root.insert(2);
        root.insert(4);
        root.insert(6);
        root.insert(1);
        root.insert(3);
        root.insert(5);
        let mut even_values = Vec::new();
        root.traverse_left_side(&mut |&value| {
            even_values.push(value);
        });
        assert_eq!(even_values, vec![6, 4, 2, 0]);
        let mut odd_values = Vec::new();
        root.traverse_right_side(&mut |&value| {
            odd_values.push(value);
        });
        assert_eq!(odd_values, vec![5, 3, 1, 0]);
    }
    #[test]
    fn ex1_complex_tree() {
        let mut root = Node::new(10);
        let values = vec![8, 6, 4, 2, 7, 5, 3, 1, 9];
        for val in values {
            root.insert(val);
        }
        let mut even_values = Vec::new();
        root.traverse_left_side(&mut |&value| {
            even_values.push(value);
            assert!(is_even(value));
        });
        let mut odd_values = Vec::new();
        root.traverse_right_side(&mut |&value| {
            if value != 10 {
                odd_values.push(value);
                assert!(!is_even(value));
            }
        });
    }
}
