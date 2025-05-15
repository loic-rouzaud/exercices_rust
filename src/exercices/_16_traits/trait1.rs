pub trait Display {
    // TODO: Définissez une méthode 'display' qui retourne une chaîne de caractères (String)
    // Cette méthode doit être implémentée par toutes les structures qui utilisent ce trait
    todo!();
}

pub struct Person {
    pub name: String,
    pub age: u32,
}

pub struct Product {
    pub name: String,
    pub price: f64,
}

impl Display for Person {
    // TODO: Implémentez la méthode display pour la structure Person
    // Elle doit retourner une chaîne au format: "Personne: [nom] ([age] ans)"
    // Exemple: "Personne: Alice (30 ans)"
    todo!()
}

impl Display for Product {
    // TODO: Implémentez la méthode display pour la structure Product
    // Elle doit retourner une chaîne au format: "Produit: [nom] ([prix]€)"
    // Exemple: "Produit: Ordinateur (999.99€)"
    todo!()
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_display() {
        let person = Person {
            name: String::from("Alice"),
            age: 30,
        };
        assert_eq!(person.display(), "Personne: Alice (30 ans)");
    }

    #[test]
    fn test_product_display() {
        let product = Product {
            name: String::from("Ordinateur"),
            price: 999.99,
        };
        assert_eq!(product.display(), "Produit: Ordinateur (999.99€)");
    }

    #[test]
    fn test_person_attributes() {
        let person = Person {
            name: String::from("Bob"),
            age: 25,
        };
        assert_eq!(person.name, "Bob");
        assert_eq!(person.age, 25);
    }

    #[test]
    fn test_product_attributes() {
        let product = Product {
            name: String::from("Téléphone"),
            price: 599.99,
        };
        assert_eq!(product.name, "Téléphone");
        assert_eq!(product.price, 599.99);
    }

    #[test]
    fn test_display_trait_compatibility() {
        let displays: Vec<Box<dyn Display>> = vec![
            Box::new(Person {
                name: String::from("Charlie"),
                age: 40,
            }),
            Box::new(Product {
                name: String::from("Tablette"),
                price: 349.99,
            }),
        ];

        assert_eq!(displays[0].display(), "Personne: Charlie (40 ans)");
        assert_eq!(displays[1].display(), "Produit: Tablette (349.99€)");
    }
}
