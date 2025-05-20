// TODO(): Définir une structure Rectangle avec deux champs: width et height, tous deux de type f64.
// Ensuite, implémenter les fonctions suivantes:

// Cette fonction crée et retourne un nouveau Rectangle avec les dimensions données
pub fn create_rectangle(width: f64, height: f64) -> Rectangle {
    // Votre code ici
    unimplemented!() // À remplacer
}

// Cette fonction calcule et retourne l'aire du rectangle (width * height)
fn calculate_area(rectangle: &Rectangle) -> f64 {
    // Votre code ici
}

// Cette fonction calcule et retourne le périmètre du rectangle (2 * (width + height))
fn calculate_perimeter(rectangle: &Rectangle) -> f64 {
    // Votre code ici
}

fn main() {
    // Vous pouvez tester vos fonctions ici
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rectangle() {
        let rect = create_rectangle(4.0, 5.0);
        assert_eq!(rect.width, 4.0);
        assert_eq!(rect.height, 5.0);
    }

    #[test]
    fn test_calculate_area() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(calculate_area(&rect), 12.0);
    }

    #[test]
    fn test_calculate_perimeter() {
        let rect = Rectangle {
            width: 2.0,
            height: 3.0,
        };
        assert_eq!(calculate_perimeter(&rect), 10.0);
    }
}
