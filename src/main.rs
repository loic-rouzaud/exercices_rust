use ansi_term::Colour;
mod binary_tree;
use binary_tree::Node;
mod rc_smart_pointer;
use rc_smart_pointer::create_shared_data;

fn exo1() {
    let mut root = Node::new(5);

    for i in 1..100 {
        root.insert(i);
    }

    let mut values = Vec::new();
    root.traverse(&mut |value| values.push(*value));
    println!("{:?}", values)
}

fn exo2() {
    create_shared_data();
}

fn main() {
    println!("{}", Colour::Green.paint("Exercice 1"));
    exo1();
    println!("");
    println!("{}", Colour::Green.paint("Exercice 2"));
    exo2();
}
