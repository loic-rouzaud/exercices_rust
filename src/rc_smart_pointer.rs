//
// Implémentez une fonction qui crée trois variables pointant
// vers la même donnée et retourne ces trois pointeurs.
//

use std::rc::Rc;

pub fn create_shared_data() -> (Rc<String>, Rc<String>, Rc<String>) {
    let shared_str_a = Rc::new(String::from("Share that"));
    let shared_str_b = Rc::clone(&shared_str_a);
    let shared_str_c = Rc::clone(&shared_str_a);
    let shared_str_d = Rc::clone(&shared_str_a);

    println!("{:?}", Rc::strong_count(&shared_str_a));
    (shared_str_b, shared_str_c, shared_str_d)
}
