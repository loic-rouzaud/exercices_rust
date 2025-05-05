mod box_pointers {
    pub mod binary_tree;
    pub mod cons_list;
}

mod rc_pointers {
    pub mod rc_shared_data;
    pub mod rc_smart_pointer;
}

use ansi_term::Colour;
use box_pointers::binary_tree::Node;
use box_pointers::cons_list::{create_list, display_cons_list, sum_list};
use rc_pointers::rc_shared_data::{add_consumer, create_shared_resource};
use rc_pointers::rc_smart_pointer::create_shared_data;
use std::rc::Rc;

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

// Exercice 4
fn exo4() {
    let original_vec = create_shared_resource();
    println!(
        "Nombre initial de références: {}",
        Rc::strong_count(&original_vec)
    );

    println!(
        "Après 1er consommateur: {}",
        add_consumer(Rc::clone(&original_vec))
    );
    println!(
        "Après 2ème consommateur: {}",
        add_consumer(Rc::clone(&original_vec))
    );
    println!(
        "Après 3ème consommateur: {}",
        add_consumer(Rc::clone(&original_vec))
    );

    println!("{}", Colour::Blue.paint("-------------"));

    {
        let _temp_consumer = Rc::clone(&original_vec);
        println!(
            "Dans le bloc avec référence temporaire: {}",
            Rc::strong_count(&original_vec)
        );
    }
    println!("{}", Colour::Blue.paint("-------------"));

    println!("Après la fin du bloc: {}", Rc::strong_count(&original_vec));
    println!("Contenu du vecteur partagé: {:?}", *original_vec);
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

    println!("");
    println!("{}", Colour::Green.paint("Exercice 4"));
    exo4();
}
