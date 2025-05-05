use ansi_term::Colour;

mod solutions {}

mod box_pointers {
    pub mod binary_tree;
    pub mod cons_list;
}

mod rc_pointers {
    pub mod rc_smart_pointer;
}

use box_pointers::binary_tree::Node;
use box_pointers::cons_list::{create_list, display_cons_list, sum_list};
use rc_pointers::rc_smart_pointer::create_shared_data;

// Exercice 1
fn exo1() {
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

// Exercice 2
fn exo2() {
    create_shared_data();
}

// Exercice 3
fn exo3() {
    let elements = vec![1, 2, 3, 4, 5];
    let my_list = create_list(elements);
    println!("{}", display_cons_list(&my_list));
    println!("{}", sum_list(&my_list));
}

// Main de test
fn main() {
    println!("{}", Colour::Green.paint("Exercice 1"));
    exo1();
    println!("");
    println!("{}", Colour::Green.paint("Exercice 2"));
    exo2();
    println!("");
    println!("{}", Colour::Green.paint("Exercice 3"));
    exo3();
}
