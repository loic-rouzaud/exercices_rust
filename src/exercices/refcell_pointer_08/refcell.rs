use std::cell::RefCell;

// Exercice difficile

// Prototype de la structure
pub struct Logger {
    log: RefCell<Vec<String>>,
}

impl Logger {
    pub fn new() -> Self {
        // TODO() : Initialiser un nouveau Logger avec un vecteur vide
        // enveloppé dans un RefCell pour permettre la mutabilité intérieure
    }

    pub fn log(&self, message: &str) {
        // TODO() : Ajouter un message au journal en empruntant mutablement
        // le vecteur contenu dans le RefCell
    }

    pub fn get_logs(&self) -> Vec<String> {
        // TODO() : Obtenir une copie des logs actuels en empruntant
        // le vecteur de manière non-mutable et en le clonant
    }

    pub fn clear(&self) {
        // TODO() : Effacer tous les logs en empruntant mutablement
        // le vecteur et en appelant la méthode clear()
    }

    pub fn process_with_logging<T>(&self, data: T, processor: impl Fn(T, &dyn Fn(&str)) -> T) -> T {
        // TODO() : Implémenter une fonction de traitement générique qui:
        // 1. Définit une closure pour enregistrer des messages
        // 2. Appelle la fonction processor fournie avec les données et la closure
        // 3. Retourne le résultat du traitement
    }
}

fn main() {
    // pour tester vos fonctions
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
