// ⚠️
// FICHIER QUI N'EST PAS DESTINÉ À RESTER EN L'ETAT, IL ME PERMET JUSTE DE TESTER
// MES FONCTIONS DE MON COTÉ.

use crate::box_pointers::binary_tree::Node;
use crate::box_pointers::cons_list::{create_list, display_cons_list, sum_list};
use crate::error_handling::user_option::User;
use crate::fn_pointers::event_manager::EventManager;
use crate::fn_pointers::fnmut::apply_operations;
use crate::hashmaps::hashmap::{count_chars, word_frequency};
use crate::loops::iterations::{
    square_roots_of_even, sum_even_numbers_like_c, sum_even_numbers_like_rust, to_uppercase,
    unique_words,
};
use crate::mutex_pointers::mutex::{create_counter, increment_counter};
use crate::rc_pointers::rc_shared_data::{add_consumer, create_shared_resource};
use crate::rc_pointers::rc_smart_pointer::create_shared_data;
use crate::refcell_pointer::refcell::Logger;
use crate::traits::trait1::{Display, Person, Product};
use crate::traits::trait2::{analyze_shape, Circle, Rectangle};

use ansi_term::Colour;
use std::rc::Rc;
use std::sync::Arc;

// Exercice Binary tree
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

// Exercice Rc_pointers
pub fn exo2() {
    create_shared_data();
}

// Exercice Cons List
pub fn exo3() {
    let elements = vec![1, 2, 3, 4, 5];
    let my_list = create_list(elements);
    println!("{}", display_cons_list(&my_list));
    println!("{}", sum_list(&my_list));
}

// Exercice Rc pointers
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

// Exercice closures fn -> ()
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

// Exercice Arc pointers
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

// Exercices fnMut fnOnce et fn
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

// Exercice
pub fn exo8() {
    let logger = Logger::new();

    let texte_resultat =
        logger.process_with_logging(String::from("Bonjour"), |mut texte, log_fn| {
            log_fn("Ici on pousse ça dans le vecteur -> Traitement de texte commencé");
            texte.push_str(" à vous");
            log_fn(&format!("Texte après modification: {}", texte));
            texte
        });
    let prouts = logger.get_logs();
    for prout in prouts {
        println!("{prout}")
    }
    println!("\nClearing logger....");
    logger.clear();
    println!("Logs after clearing : {}", texte_resultat);
}

// Exercice iter() / map() / filter()
pub fn exo9() {
    let sentence_vec = vec![
        "Bonjour je m'appelle Loïc",
        "Bonjour je suis un alien",
        "Salut comment vas-tu",
    ];

    let arr = [String::from("hello"), String::from("world")];

    let num_vec = vec![1, 2, 3, 4, 5];

    println!("sum like C : {:?}", sum_even_numbers_like_c(0, 49));
    println!("sum like Rust : {:?}", sum_even_numbers_like_rust(0, 49));

    let words_vec: Vec<String> = unique_words(sentence_vec);
    println!("\nVecteur après : {:?}\n", words_vec);

    square_roots_of_even(num_vec);

    println!("Ici le vecteur en uppercase {:#?}", to_uppercase(&arr));
}

// Exercice
pub fn exo10() {
    let user = User::new(String::from("alice"));
    println!("Nouvel utilisateur créé : {}\n", user.get_contact_info());

    let user_with_email = user.with_email(String::from("alice@example.com"));
    println!(
        "Après ajout de l'email : {}",
        user_with_email.get_contact_info()
    );

    let young_user = User::new(String::from("bob"))
        .with_email(String::from("bob@example.com"))
        .with_age(16);
    println!(
        "Jeune utilisateur créé : {}\n",
        young_user.get_contact_info()
    );

    match young_user.is_adult() {
        Some(true) => println!("Bob est adulte"),
        Some(false) => println!("Bob est mineur"),
        None => println!("Âge de Bob non spécifié"),
    }

    let adult_user = User::new(String::from("charlie")).with_age(25);
    println!(
        "Utilisateur adulte créé : {}\n",
        adult_user.get_contact_info()
    );

    match adult_user.is_adult() {
        Some(true) => println!("Charlie est adulte"),
        Some(false) => println!("Charlie est mineur"),
        None => println!("Âge de Charlie non spécifié"),
    }

    let unknown_age_user =
        User::new(String::from("david")).with_email(String::from("david@example.com"));
    println!(
        "Utilisateur sans âge : {}\n",
        unknown_age_user.get_contact_info()
    );

    match unknown_age_user.is_adult() {
        Some(true) => println!("David est adulte"),
        Some(false) => println!("David est mineur"),
        None => println!("Âge de David non spécifié"),
    }
}

// Exercices hashmap
pub fn exo11() {
    let str = "Lorem ipsum prout Lorem ipsum prout Lorem ipsum prout Lorem ipsum prout";

    println!("{:?}", count_chars(str));
    println!("{:?}", word_frequency(str));
}

// Exercices sur les traits
pub fn exo12() {
    let personne = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let produit = Product {
        name: String::from("Ordinateur"),
        price: 999.99,
    };

    personne.display();
    produit.display();
}

pub fn exo13() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    analyze_shape(&circle);
    analyze_shape(&rectangle);
}
