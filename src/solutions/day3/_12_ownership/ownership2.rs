fn add_to_vec(vec: &mut Vec<i32>) {
    vec.push(88);
}

fn main() {
    let mut v = vec![1, 2, 3];
    add_to_vec(&mut v);
    println!("{:?}", v);
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
