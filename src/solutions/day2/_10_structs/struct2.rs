#[derive(Debug)]
struct Product {
    name: String,
    price: u32, // prix en centimes
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: u32, in_stock: bool) -> Self {
        if price == 0 {
            // Ce n'est pas la façon dont vous devriez gérer les erreurs en Rust,
            // mais nous apprendrons la gestion des erreurs plus tard.
            panic!("Le prix d'un produit ne peut pas être zéro");
        }
        Self {
            name,
            price,
            in_stock,
        }
    }

    fn is_available(&self) -> bool {
        self.in_stock
    }

    fn get_price_with_tax(&self, tax_percentage: u32) -> u32 {
        self.price + (self.price * tax_percentage / 100)
    }
}

fn main() {
    let product = Product::new(String::from("Ordinateur portable"), 120000, true);
    println!("Produit: {:?}", product);
    println!("Disponible: {}", product.is_available());
    println!("Prix avec taxe (20%): {}", product.get_price_with_tax(20));
}
