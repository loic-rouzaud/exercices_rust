pub trait Display {
    fn display(&self);
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
    fn display(&self) {
        println!("Personne: {} ({} ans)", self.name, self.age);
    }
}

impl Display for Product {
    fn display(&self) {
        println!("Produit: {} ({}€)", self.name, self.price);
    }
}
