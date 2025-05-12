use std::cell::RefCell;

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
