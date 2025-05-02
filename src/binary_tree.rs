//
// Exercice 1
//
// Goal : Understand how Box smart pointers enable recursive data structures
// in Rust by examining a binary tree implementation that organizes
// values based on their properties.
//

pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    pub fn traverse_left_side(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(left) = &self.left {
            left.traverse_left_side(visit);
        }
        visit(&self.value);
    }

    pub fn traverse_right_side(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(right) = &self.right {
            right.traverse_right_side(visit);
        }
        visit(&self.value);
    }
}

impl Node<i32> {
    pub fn insert(&mut self, value: i32) {
        if is_even(value) {
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(left_child) => left_child.insert(value),
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(right_child) => right_child.insert(value),
            }
        }
    }
}
