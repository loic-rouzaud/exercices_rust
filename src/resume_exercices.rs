use crate::exercices::box_pointers_04::binary_tree::Node;
use crate::exercices::box_pointers_04::cons_list::{create_list, display_cons_list, sum_list};
use crate::exercices::error_handling_02::user_option::User;
use crate::exercices::fn_pointers_05::event_manager::EventManager;
use crate::exercices::fn_pointers_05::fnmut::apply_operations;
use crate::exercices::hashmaps_03::hashmap::{count_chars, word_frequency};
use crate::exercices::loops_01::iterations::{
    square_roots_of_even, sum_even_numbers_like_c, sum_even_numbers_like_rust, to_uppercase,
    unique_words,
};
use crate::exercices::mutex_pointers_06::mutex::{create_counter, increment_counter};
use crate::exercices::rc_pointers_07::rc_shared_data::{add_consumer, create_shared_resource};
use crate::exercices::rc_pointers_07::rc_smart_pointer::create_shared_data;
use crate::exercices::refcell_pointer_08::refcell::Logger;

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

// Exercice 8
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

// Exercice 9
// Simplement pour revoir les methodes d'iterations ect...
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

// Exercice 10
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

pub fn exo11() {
    let str = "Lorem ipsum prout Lorem ipsum prout Lorem ipsum prout Lorem ipsum prout";

    println!("{:?}", count_chars(str));
    println!("{:?}", word_frequency(str));
}
