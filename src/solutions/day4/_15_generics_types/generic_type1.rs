fn swap<T>(a: &mut T, b: &mut T) {
    let temp = std::mem::replace(a, std::mem::replace(b, std::mem::take(a)));
    *b = temp;
}

fn main() {
    // Test avec des entiers
    let mut a = 5;
    let mut b = 10;
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);

    // Test avec des chaînes de caractères
    let mut s1 = String::from("bonjour");
    let mut s2 = String::from("rust");
    swap(&mut s1, &mut s2);
    println!("s1 = {}, s2 = {}", s1, s2);

    // Test avec des booléens
    let mut flag1 = true;
    let mut flag2 = false;
    swap(&mut flag1, &mut flag2);
    println!("flag1 = {}, flag2 = {}", flag1, flag2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_integers() {
        let mut x = 5;
        let mut y = 10;
        swap(&mut x, &mut y);
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_swap_strings() {
        let mut s1 = String::from("hello");
        let mut s2 = String::from("world");
        swap(&mut s1, &mut s2);
        assert_eq!(s1, "world");
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_swap_floats() {
        let mut f1 = 3.14;
        let mut f2 = 2.71;
        swap(&mut f1, &mut f2);
        assert_eq!(f1, 2.71);
        assert_eq!(f2, 3.14);
    }
}
