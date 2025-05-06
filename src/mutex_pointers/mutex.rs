use std::sync::{Arc, Mutex};
use std::thread;

pub fn create_counter() -> Arc<Mutex<i32>> {
    let mutex = Arc::new(Mutex::new(0));
    mutex
}

pub fn increment_counter(counter: Arc<Mutex<i32>>, increments: i32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut counter_guard = counter.lock().unwrap();
        *counter_guard += increments;
        println!("thread passé")
    })
}
