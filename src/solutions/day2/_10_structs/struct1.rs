pub struct Rectangle {
    width: f64,
    height: f64,
}

pub fn create_rectangle(width: f64, height: f64) -> Rectangle {
    Rectangle { width, height }
}

fn calculate_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

fn calculate_perimeter(rectangle: &Rectangle) -> f64 {
    2.0 * (rectangle.width + rectangle.height)
}

fn main() {
    let rect = create_rectangle(5.0, 10.0);
    println!("Area: {}", calculate_area(&rect));
    println!("Perimeter: {}", calculate_perimeter(&rect));
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
