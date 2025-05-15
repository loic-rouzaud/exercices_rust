use std::fmt::Display;

//
// Exercice difficile
// Implementer une liste chainée
//

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
    // à partir d'un vecteur d'éléments
}

pub fn display_cons_list<T: Display>(list: &List<T>) -> String {
    // TODO() : Implémenter la fonction display_cons_list qui convertit la liste
    // en une chaîne de caractères pour l'affichage
}

pub fn sum_list(list: &List<i32>) -> i32 {
    // TODO() : Implémenter la fonction sum_list qui additionne tous les
    // éléments d'une liste d'entiers
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_function() {
        let list = cons(3, cons(2, cons(1, List::Nil)));
        match list {
            List::Cons(head, tail) => {
                assert_eq!(head, 3);
                match *tail {
                    List::Cons(head, _) => assert_eq!(head, 2),
                    List::Nil => panic!("Expected non-empty tail"),
                }
            }
            List::Nil => panic!("Expected non-empty list"),
        }
    }

    #[test]
    fn test_create_list() {
        let elements = vec![1, 2, 3];
        let list = create_list(elements);

        // (indice)
        // La liste devrait être construite en ordre inverse: 3 -> 2 -> 1 -> Nil
        match list {
            List::Cons(head, tail) => {
                assert_eq!(head, 3);
                match *tail {
                    List::Cons(head, tail) => {
                        assert_eq!(head, 2);
                        match *tail {
                            List::Cons(head, tail) => {
                                assert_eq!(head, 1);
                                match *tail {
                                    List::Nil => {}
                                    _ => panic!("Expected Nil at end of list"),
                                }
                            }
                            _ => panic!("Expected third element"),
                        }
                    }
                    _ => panic!("Expected second element"),
                }
            }
            List::Nil => panic!("Expected non-empty list"),
        }
    }

    #[test]
    fn test_display_cons_list() {
        let list = cons(3, cons(2, cons(1, List::Nil)));
        assert_eq!(display_cons_list(&list), "3 -> 2 -> 1 -> Nil");

        let empty: List<i32> = List::Nil;
        assert_eq!(display_cons_list(&empty), "Nil");
    }

    #[test]
    fn test_sum_list() {
        let list = cons(5, cons(10, cons(15, List::Nil)));
        assert_eq!(sum_list(&list), 30);

        let empty: List<i32> = List::Nil;
        assert_eq!(sum_list(&empty), 0);
    }
}
