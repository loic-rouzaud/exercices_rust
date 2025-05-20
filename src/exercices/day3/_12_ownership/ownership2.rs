// TODO: Corrige l'erreur de compilation dans cette fonction.
// Indice: Comment pouvons-nous modifier le vecteur sans en prendre possession?
fn add_to_vec(vec: Vec<i32>) {
    vec.push(88);
}

fn main() {
    // Tu peux expérimenter ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_and_borrow() {
        let mut vec0 = vec![22, 44, 66];
        add_to_vec(vec0);

        // Après avoir appelé add_to_vec, le vecteur devrait contenir un élément supplémentaire
        assert_eq!(vec0, vec![22, 44, 66, 88]);
    }
}
