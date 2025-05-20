use std::sync::{Arc, Mutex};
use std::thread;

fn create_counter() -> Arc<Mutex<i32>> {
    let mutex = Arc::new(Mutex::new(0));
    mutex
}

fn increment_counter(counter: Arc<Mutex<i32>>, increments: i32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut counter_guard = counter.lock().unwrap();
        *counter_guard += increments;
        println!("thread passé")
    })
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
