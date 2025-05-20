// TODO: Corrige l'erreur de compilation dans cette fonction.
// Indice: Réfléchis à qui est propriétaire du vecteur.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;
    vec.push(88);
    vec
}

fn main() {
    // Tu peux expérimenter ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // Ceci devrait fonctionner
        assert_eq!(vec1, vec![22, 44, 66, 88]);
        // TODO: Essaie d'utiliser vec0 ici. Que se passe-t-il?
        // Décommente la ligne suivante:
        // println!("vec0: {:?}", vec0);
    }
}
