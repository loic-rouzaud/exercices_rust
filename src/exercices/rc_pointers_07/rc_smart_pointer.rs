//
// Comprendre comment les pointeurs intelligents Rc permettent
// la propriété partagée des données en Rust en implémentant une fonction
// qui crée plusieurs références vers les mêmes données.
//
use std::rc::Rc;

pub fn create_shared_data() -> (Rc<String>, Rc<String>, Rc<String>) {
    // TODO() : Créer une String enveloppée dans un Rc pour permettre le partage
    // Créer plusieurs clones du Rc (pas de la String elle-même)
    // Afficher le nombre de références fortes
    // Afficher le contenu de la String partagée
    // Retourner un tuple contenant trois références à la même donnée
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex2_shared_references() {
        let (ref1, ref2, ref3) = create_shared_data();
        assert_eq!(*ref1, "I'm another variable !!");
        assert_eq!(*ref2, "I'm another variable !!");
        assert_eq!(*ref3, "I'm another variable !!");
        assert_eq!(Rc::strong_count(&ref1), 3);
    }

    #[test]
    fn ex2_reference_counting() {
        let (ref1, ref2, _ref3) = create_shared_data(); // Correction: *ref3 -> _ref3
        assert_eq!(Rc::strong_count(&ref1), 3);
        {
            let _ref4 = Rc::clone(&ref1);
            assert_eq!(Rc::strong_count(&ref1), 4);
        }
        assert_eq!(Rc::strong_count(&ref1), 3);
        drop(ref2);
        assert_eq!(Rc::strong_count(&ref1), 2);
    }

    #[test]
    fn ex2_data_immutability() {
        let (ref1, _, _) = create_shared_data(); // Correction: *, * -> _, _
        assert_eq!(*ref1, "I'm another variable !!");
    }
}
