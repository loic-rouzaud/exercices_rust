fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}

fn main() {
    let v = vec![1, 2, 3];
    let v = fill_vec(v);
    println!("{:?}", v);
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
