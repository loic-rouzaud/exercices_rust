use ansi_term::Colour;
mod binary_tree;
use binary_tree::Node;
mod rc_smart_pointer;
use rc_smart_pointer::create_shared_data;

fn exo1() {
    let mut even_values = Vec::new();
    let mut odd_values = Vec::new();
    let mut root = Node::new(0);

    for i in 1..50 {
        root.insert(i);
    }

    root.traverse_left_side(&mut |value| even_values.push(*value));
    println!("even values : {:?}", even_values);
    root.traverse_right_side(&mut |value| odd_values.push(*value));
    println!("odd_values : {:?}", odd_values);
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
