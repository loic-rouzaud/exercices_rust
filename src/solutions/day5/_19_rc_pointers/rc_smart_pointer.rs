use std::rc::Rc;

pub fn create_shared_data() -> (Rc<String>, Rc<String>, Rc<String>) {
    let shared_str_a = Rc::new(String::from("I'm another variable !!"));
    let shared_str_b = Rc::clone(&shared_str_a);
    let shared_str_c = Rc::clone(&shared_str_a);
    let shared_str_d = Rc::clone(&shared_str_a);

    println!("{:?}", Rc::strong_count(&shared_str_a));
    println!("Shared data from another place : {}", shared_str_d);
    (shared_str_b, shared_str_c, shared_str_d)
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
        let (ref1, ref2, _ref3) = create_shared_data();
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
