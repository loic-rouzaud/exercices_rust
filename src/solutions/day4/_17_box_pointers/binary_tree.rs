struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn traverse_left_side(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(left) = &self.left {
            left.traverse_left_side(visit);
        }
        visit(&self.value);
    }

    fn traverse_right_side(&self, visit: &mut dyn FnMut(&T)) {
        if let Some(right) = &self.right {
            right.traverse_right_side(visit);
        }
        visit(&self.value);
    }
}

impl Node<i32> {
    fn insert(&mut self, value: i32) {
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

fn main() {
    let mut even_values = Vec::new();
    let mut odd_values = Vec::new();
    let mut root = Node::new(0);

    for i in 0..49 {
        root.insert(i);
    }

    root.traverse_left_side(&mut |value| even_values.push(*value));
    println!("even values : {:?}", even_values);
    root.traverse_right_side(&mut |value| odd_values.push(*value));
    println!("odd_values : {:?}", odd_values);
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
