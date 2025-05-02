//
// Exercice 1
// Goal : create a binary tree, that sorts even values
// on the right, and others on the left.
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

    pub fn traverse(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(left) = &self.left {
            left.traverse(visit);
        }

        visit(&self.value);

        if let Some(right) = &self.right {
            right.traverse(visit);
        }
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
