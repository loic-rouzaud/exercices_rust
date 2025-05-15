use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn display_info(&self) {
        // TODO: Implémentez cette méthode pour afficher l'aire et le périmètre de la forme
        // en utilisant les méthodes area() et perimeter()
        // Format attendu: "Area: X.XX, Perimeter: Y.YY"
        todo!()
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        // TODO: Calculez l'aire du cercle en utilisant la formule: π × r²
        // où r est le rayon du cercle et π est accessible via std::f64::consts::PI
        todo!()
    }

    fn perimeter(&self) -> f64 {
        // TODO: Calculez le périmètre (circonférence) du cercle en utilisant la formule: 2 × π × r
        // où r est le rayon du cercle et π est accessible via std::f64::consts::PI
        todo!()
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        // TODO: Calculez l'aire du rectangle en utilisant la formule: largeur × hauteur
        todo!()
    }

    fn perimeter(&self) -> f64 {
        // TODO: Calculez le périmètre du rectangle en utilisant la formule: 2 × (largeur + hauteur)
        todo!()
    }
}

pub fn analyze_shape<T: Shape>(shape: &T) {
    // TODO: Implémentez cette fonction pour afficher "Shape analysis:" suivi
    // d'un appel à la méthode display_info() de la forme
    todo!()
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.area(), PI * 25.0);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle { radius: 5.0 };
        assert_eq!(circle.perimeter(), 2.0 * PI * 5.0);
    }

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle {
            width: 10.0,
            height: 5.0,
        };
        assert_eq!(rectangle.area(), 50.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle {
            width: 10.0,
            height: 5.0,
        };
        assert_eq!(rectangle.perimeter(), 30.0);
    }

    #[test]
    fn test_shape_trait_compatibility() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 3.0 }),
            Box::new(Rectangle {
                width: 4.0,
                height: 6.0,
            }),
        ];

        assert!((shapes[0].area() - (PI * 9.0)).abs() < 1e-10);
        assert!((shapes[1].area() - 24.0).abs() < 1e-10);
    }
}
