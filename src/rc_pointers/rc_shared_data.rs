use std::rc::Rc;

// À implémenter
pub fn create_shared_resource() -> Rc<Vec<i32>> {
    // Créer et retourner un Rc contenant un vecteur
    let my_vec = vec![1, 2, 3, 4, 5];
    let shared_data = Rc::new(my_vec);
    shared_data
}

pub fn add_consumer(resource: Rc<Vec<i32>>) -> usize {
    // Créer un "consommateur" qui possède une référence au resource
    // Retourner le nombre de références actives (strong_count)
    let _consumer = Rc::clone(&resource);
    Rc::strong_count(&resource)
}
