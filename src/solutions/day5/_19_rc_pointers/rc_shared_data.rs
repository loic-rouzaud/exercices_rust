use std::rc::Rc;

fn create_shared_resource() -> Rc<Vec<i32>> {
    let my_vec = vec![1, 2, 3, 4, 5];
    let shared_data = Rc::new(my_vec);
    shared_data
}

fn add_consumer(resource: &Rc<Vec<i32>>) -> usize {
    let _consumer = Rc::clone(resource);
    Rc::strong_count(resource)
}

fn main() {
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

    {
        let _temp_consumer = Rc::clone(&original_vec);
        println!(
            "Dans le bloc avec référence temporaire: {}",
            Rc::strong_count(&original_vec)
        );
    }

    println!("Après la fin du bloc: {}", Rc::strong_count(&original_vec));
    println!("Contenu du vecteur partagé: {:?}", *original_vec);
}

#[test]
fn test_shared_resource() {
    let original_vec = create_shared_resource();
    assert_eq!(Rc::strong_count(&original_vec), 1);
    let count1 = add_consumer(&original_vec);
    assert_eq!(count1, 2); // ⚠️ ATTENTION: Le test actuel a une erreur, il devrait être 2 et non 3
    let count2 = add_consumer(&original_vec);
    assert_eq!(count2, 3);
}
