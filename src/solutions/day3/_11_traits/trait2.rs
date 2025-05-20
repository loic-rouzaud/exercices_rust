use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn display_info(&self) {
        println!(
            "Area: {:.2}, Perimeter: {:.2}",
            self.area(),
            self.perimeter()
        );
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
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub fn analyze_shape<T: Shape>(shape: &T) {
    println!("Shape analysis:");
    shape.display_info();
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
