//
// Exercice 1
//
// Goal : Understand how Box smart pointers enable recursive data structures
// in Rust by examining a binary tree implementation that organizes
// values based on their properties.
//
// Le but de l'exercice est de coder un arbre binaire qui mettra à gauche les nombres
// paires, et à droite les nombres impaires
//
// Documentation : Box<T>
//

// Ne pas changer cette structure
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Todo: paire / impaire
pub fn is_even(value: i32) -> bool {}

// impl d'un type T generique
impl<T> Node<T> {
    pub fn new(value: T) -> Self {}

    pub fn traverse_left_side(&self, visit: &mut dyn FnMut(&T)) {}

    pub fn traverse_right_side(&self, visit: &mut dyn FnMut(&T)) {}
}

// impl d'un type i32 pour trier les nombres
impl Node<i32> {
    pub fn insert(&mut self, value: i32) {}
}

// Aidez vous des tests afin de tester vos fonctions
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
