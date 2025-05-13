use std::rc::Rc;

pub fn create_shared_resource() -> Rc<Vec<i32>> {
    // TODO() : Créer un vecteur d'entiers et le transformer en une ressource
    // partagée en l'enveloppant dans un Rc (Reference Counted smart pointer)
}

pub fn add_consumer(resource: &Rc<Vec<i32>>) -> usize {
    // TODO() : Créer un nouveau consommateur en clonant le Rc
    // (ce qui incrémente le compteur de références)
    // Retourner le nombre actuel de références fortes
}

fn main() {
    // pour tester vos fonctions
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
