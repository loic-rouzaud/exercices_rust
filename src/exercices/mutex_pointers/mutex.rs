use std::sync::{Arc, Mutex};
use std::thread;

pub fn create_counter() -> Arc<Mutex<i32>> {
    // TODO() : Créer et retourner un compteur partagé (entier) protégé par un Mutex
    // et enveloppé dans un Arc pour permettre le partage entre threads
}

pub fn increment_counter(counter: Arc<Mutex<i32>>, increments: i32) -> thread::JoinHandle<()> {
    // TODO() : Lancer un nouveau thread qui incrémente le compteur partagé
    // de la valeur spécifiée. Verrouiller le Mutex, modifier la valeur et
    // retourner le handle du thread
}
