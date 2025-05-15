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

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_counter() {
        let counter = create_counter();
        let value = *counter.lock().unwrap();
        assert_eq!(value, 0);
    }

    #[test]
    fn test_increment_counter_single() {
        let counter = create_counter();
        let handle = increment_counter(Arc::clone(&counter), 5);

        handle.join().unwrap();

        let value = *counter.lock().unwrap();
        assert_eq!(value, 5);
    }

    #[test]
    fn test_increment_counter_multiple() {
        let counter = create_counter();

        let handles = vec![
            increment_counter(Arc::clone(&counter), 3),
            increment_counter(Arc::clone(&counter), 2),
            increment_counter(Arc::clone(&counter), 5),
        ];

        for handle in handles {
            handle.join().unwrap();
        }

        let value = *counter.lock().unwrap();
        assert_eq!(value, 10);
    }
}
