use std::cell::RefCell;

// Prototype de la structure
pub struct Logger {
    log: RefCell<Vec<String>>,
}

// Prototypes des fonctions à implémenter
impl Logger {
    pub fn new() -> Self {
        Logger {
            log: RefCell::new(Vec::new()),
        }
    }

    pub fn log(&self, message: &str) {
        self.log.borrow_mut().push(String::from(message));
    }

    pub fn get_logs(&self) -> Vec<String> {
        let logs = self.log.borrow().clone();
        logs
    }

    pub fn clear(&self) {
        self.log.borrow_mut().clear();
    }

    pub fn process_with_logging<T>(&self, data: T, processor: impl Fn(T, &dyn Fn(&str)) -> T) -> T {
        let log_function = |msg: &str| {
            self.log(msg);
        };
        processor(data, &log_function)
    }
}
