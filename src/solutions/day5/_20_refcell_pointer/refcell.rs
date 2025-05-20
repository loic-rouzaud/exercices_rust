use std::cell::RefCell;

struct Logger {
    log: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log: RefCell::new(Vec::new()),
        }
    }

    fn log(&self, message: &str) {
        self.log.borrow_mut().push(String::from(message));
    }

    fn get_logs(&self) -> Vec<String> {
        let logs = self.log.borrow().clone();
        logs
    }

    fn clear(&self) {
        self.log.borrow_mut().clear();
    }

    fn process_with_logging<T>(&self, data: T, processor: impl Fn(T, &dyn Fn(&str)) -> T) -> T {
        let log_function = |msg: &str| {
            self.log(msg);
        };
        processor(data, &log_function)
    }
}

fn main() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_basic() {
        let logger = Logger::new();
        logger.log("Message 1");
        logger.log("Message 2");

        let logs = logger.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "Message 1");
        assert_eq!(logs[1], "Message 2");
    }

    #[test]
    fn test_logger_clear() {
        let logger = Logger::new();
        logger.log("Message 1");
        logger.clear();

        let logs = logger.get_logs();
        assert_eq!(logs.len(), 0);
    }

    #[test]
    fn test_process_with_logging() {
        let logger = Logger::new();

        let result = logger.process_with_logging(5, |value, log_fn| {
            log_fn("Starting processing");
            let new_value = value * 2;
            log_fn(&format!("Multiplied by 2: {}", new_value));
            new_value
        });

        assert_eq!(result, 10);

        let logs = logger.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "Starting processing");
        assert_eq!(logs[1], "Multiplied by 2: 10");
    }
}
