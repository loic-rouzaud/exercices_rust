use std::fmt::Display;

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn cons<T>(x: T, list: List<T>) -> List<T> {
    // TODO() : Implémenter la fonction cons qui crée un nouveau nœud Cons
    // contenant la valeur x et pointant vers la liste existante
}

pub fn create_list<T>(elements: Vec<T>) -> List<T> {
    // TODO() : Implémenter la fonction create_list qui construit une liste chaînée
    // à partir d'un vecteur d'éléments (en ordre inverse)
}

pub fn display_cons_list<T: Display>(list: &List<T>) -> String {
    // TODO() : Implémenter la fonction display_cons_list qui convertit la liste
    // en une chaîne de caractères pour l'affichage
}

pub fn sum_list(list: &List<i32>) -> i32 {
    // TODO() : Implémenter la fonction sum_list qui additionne tous les
    // éléments d'une liste d'entiers
}
