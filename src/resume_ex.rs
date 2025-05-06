use crate::box_pointers::binary_tree::Node;
use crate::box_pointers::cons_list::{create_list, display_cons_list, sum_list};
use crate::fn_pointers::event_manager::EventManager;
use crate::fn_pointers::fnmut::apply_operations;
use crate::mutex_pointers::mutex::{create_counter, increment_counter};
use crate::rc_pointers::rc_shared_data::{add_consumer, create_shared_resource};
use crate::rc_pointers::rc_smart_pointer::create_shared_data;
use ansi_term::Colour;
use std::rc::Rc;
use std::sync::Arc;

// Exercice 1
pub fn exo1() {
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
pub fn exo2() {
    create_shared_data();
}

// Exercice 3
pub fn exo3() {
    let elements = vec![1, 2, 3, 4, 5];
    let my_list = create_list(elements);
    println!("{}", display_cons_list(&my_list));
    println!("{}", sum_list(&my_list));
}

// Exercice 4
pub fn exo4() {
    let original_vec = create_shared_resource();
    println!(
        "Nombre initial de références: {}",
        Rc::strong_count(&original_vec)
    );

    println!(
        "Après 1er consommateur: {}",
        add_consumer(&Rc::clone(&original_vec))
    );
    println!(
        "Après 2ème consommateur: {}",
        add_consumer(&Rc::clone(&original_vec))
    );
    println!(
        "Après 3ème consommateur: {}",
        add_consumer(&Rc::clone(&original_vec))
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

// Exercice 5
pub fn exo5() {
    let mut vec1 = vec![1, 2, 3, 4, 5];

    // L'usage est surtout + important que la fonction en elle-meme
    let vec2 = vec![
        // Closure sur le type
        |vec: &mut Vec<i32>| {
            for n in vec.iter_mut() {
                *n *= 10;
            }
        },
        |vec: &mut Vec<i32>| {
            vec.push(99);
        },
        |vec: &mut Vec<i32>| {
            vec.reverse();
        },
    ];

    println!("Avant {:?}", vec1);
    apply_operations(&mut vec1, vec2);
    println!("Après {:?}", vec1);
}

// Exercice 6
pub fn exo6() {
    let counter = create_counter();

    let handles = vec![
        increment_counter(Arc::clone(&counter), 1000),
        increment_counter(Arc::clone(&counter), 1000),
        increment_counter(Arc::clone(&counter), 1000),
    ];

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Valeur finale: {}", *counter.lock().unwrap());
}

pub fn exo7() {
    let mut event_manager = EventManager::new();

    event_manager.register_start_handler(|| {
        println!("Application démarrée !");
    });

    event_manager.register_start_handler(|| {
        println!("Initialisation des ressources...");
    });

    event_manager.register_message_handler(|msg| format!("Echo: {}", msg));

    event_manager.register_message_handler(|msg| format!("Majuscules: {}", msg.to_uppercase()));

    event_manager.register_shutdown_handler(|| {
        println!("Fermeture des connexions...");
        true
    });

    event_manager.register_shutdown_handler(|| {
        println!("Sauvegarde des données...");
        true
    });

    println!("=== Déclenchement de l'événement de démarrage ===");
    event_manager.trigger_start();

    println!("\n=== Traitement des messages ===");
    let message = String::from("bonjour rust");
    let responses = event_manager.process_message(message);

    for (i, response) in responses.iter().enumerate() {
        println!("Réponse du handler {}: {}", i + 1, response);
    }

    println!("\n=== Fermeture de l'application ===");
    let shutdown_successful = event_manager.shutdown();
    println!("Fermeture réussie: {}", shutdown_successful);
}
