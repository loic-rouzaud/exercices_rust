mod binary_tree;
use binary_tree::Node;

fn main() {
    let mut root = Node::new(5);

    for i in 1..100 {
        root.insert(i);
    }

    let mut values = Vec::new();
    root.traverse(&mut |value| values.push(*value));
    println!("{:?}", values)
}
